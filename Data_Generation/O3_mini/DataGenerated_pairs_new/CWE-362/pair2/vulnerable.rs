///////////////////////////////////////////////
// Vulnerable Code: Race Condition Example  //
// CWE-362: Race Condition in concurrent code //
///////////////////////////////////////////////

use std::thread;
use std::time::Duration;

struct Data {
    count: usize,
}

static mut GLOBAL: *mut Data = std::ptr::null_mut();

fn init() {
    let data = Box::new(Data { count: 0 });
    unsafe {
        GLOBAL = Box::into_raw(data);
    }
}

/// Executes the main concurrent processing and returns the final count.
/// This function uses an unsynchronized raw pointer in multiple threads.
pub fn run_process() -> usize {
    init();
    let mut handles = Vec::new();
    // Spawn 10 threads that concurrently update a shared non-atomic counter.
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            // Unsynchronized access in unsafe block
            unsafe {
                let data = GLOBAL.as_mut().expect("Not initialized");
                // A check-then-act sequence with delay to widen the race window:
                if data.count % 2 == 0 {
                    thread::sleep(Duration::from_millis(10));
                    data.count += 1;
                } else {
                    thread::sleep(Duration::from_millis(10));
                    data.count += 2;
                }
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    unsafe {
        // Final read without synchronization produces a potential race condition.
        let final_data = &*GLOBAL;
        final_data.count
    }
}

fn main() {
    let result = run_process();
    println!("Final count: {}", result);
}