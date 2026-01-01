//////////////////////////////
// Vulnerable Version -- Race Condition CWE-362
//////////////////////////////
use std::thread;
use std::time::Duration;

static mut SHARED: i32 = 0;

fn run_app() -> i32 {
    // Spawn 8 threads each doing 1000 unsynchronized increments.
    let mut handles = vec![];
    for _ in 0..8 {
        let handle = thread::spawn(|| {
            for _ in 0..1000 {
                // Unsafe unsynchronized read-modify-write leading to a race condition
                unsafe {
                    let temp = SHARED;
                    // Artificial delay to widen the window for race condition
                    thread::sleep(Duration::from_nanos(1));
                    SHARED = temp + 1;
                }
            }
        });
        handles.push(handle);
    }
    // Join all threads.
    for h in handles {
        h.join().unwrap();
    }
    // Return the final value
    unsafe { SHARED }
}

fn main() {
    let result = run_app();
    println!("Final counter value: {}", result);
}