use std::thread;
use std::time::Duration;
use std::ptr;

struct Counter {
    count: u64,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

static mut GLOBAL: *mut Counter = ptr::null_mut();

fn run_concurrent(increments: u64, threads: usize) -> u64 {
    // Initialize the global via allocation; unsafely shared across threads.
    let counter = Box::new(Counter::new());
    unsafe {
        GLOBAL = Box::into_raw(counter);
    }

    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        // Each thread unsafely increments the shared counter.
        let handle = thread::spawn(move || {
            for _ in 0..increments {
                unsafe {
                    // UNSAFE: simultaneous unsynchronized access introduces a race condition.
                    (*GLOBAL).count = (*GLOBAL).count.wrapping_add(1);
                    // Delay increases the window for context switches.
                    thread::sleep(Duration::from_micros(1));
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result;
    unsafe {
        result = (*GLOBAL).count;
        // Clean up the allocated memory.
        Box::from_raw(GLOBAL);
        GLOBAL = ptr::null_mut();
    }
    result
}

fn main() {
    let threads = 4;
    let increments = 10000;
    let final_value = run_concurrent(increments, threads);
    println!("Final count: {}", final_value);
}