//////////////////////////////// Vulnerable Version //////////////////////////////////
use std::{thread, ptr};

struct Data {
    count: u32,
}

static mut GLOBAL: *mut Data = ptr::null_mut();

fn run_system() -> u32 {
    unsafe {
        // initialize GLOBAL unsafely without proper synchronization
        let data = Box::new(Data { count: 0 });
        GLOBAL = Box::into_raw(data);
    }

    let mut threads = vec![];

    for _ in 0..10 {
        threads.push(thread::spawn(|| {
            for _ in 0..100 {
                unsafe {
                    // simulate a race condition: check and update the global count unsafely
                    if !GLOBAL.is_null() {
                        let current = (*GLOBAL).count;
                        // Introduce a yield to widen the race window
                        thread::yield_now();
                        // unsynchronized update: non-atomic read-modify-write sequence vulnerable to races
                        (*GLOBAL).count = current + 1;
                    }
                }
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    let result;
    unsafe {
        result = (*GLOBAL).count;
        // Clean up: free the allocated memory unsafely
        let _ = Box::from_raw(GLOBAL);
        GLOBAL = ptr::null_mut();
    }
    result
}

fn main() {
    let total = run_system();
    println!("Final count: {}", total);
}