use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    value: u32,
}

impl Data {
    // This method performs an unsynchronized increment.
    unsafe fn do_increment(ptr: *mut Data) {
        // Potential race: modifying shared data without proper synchronization.
        (*ptr).value = (*ptr).value.wrapping_add(1);
    }
}

fn compute() -> u32 {
    let shared = Arc::new(Mutex::new(Data { value: 0 }));
    let mut handles = vec![];

    for _ in 0..4 {
        let s = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..10000 {
                // Extract a raw pointer while holding the lock
                let raw_ptr: *mut Data = {
                    let guard = s.lock().unwrap();
                    // Exposing the raw pointer outside the lock's protection
                    &*guard as *const Data as *mut Data
                };
                // Unsafe update outside the lock – the mutex no longer guards this access
                unsafe {
                    Data::do_increment(raw_ptr);
                }
            }
        });
        handles.push(handle);
    }

    for h in handles {
        let _ = h.join();
    }
    let result = shared.lock().unwrap().value;
    result
}

fn main() {
    let result = compute();
    println!("Final value: {}", result);
}