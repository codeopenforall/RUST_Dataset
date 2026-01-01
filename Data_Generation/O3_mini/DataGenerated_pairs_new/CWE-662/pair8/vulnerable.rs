use std::sync::{Arc, Mutex};
use std::thread;

struct Shared {
    first: Mutex<i32>,
    second: Mutex<i32>,
}

impl Shared {
    fn new() -> Self {
        Self {
            first: Mutex::new(0),
            second: Mutex::new(0),
        }
    }
    
    fn update(&self) {
        // Lock both mutexes in a consistent order.
        let mut l1 = self.first.lock().unwrap();
        let mut l2 = self.second.lock().unwrap();
        *l1 += 1;
        *l2 += 1;
    }
    
    // Final computation using an unsafe unsynchronized read.
    // The code locks the first mutex but then obtains a pointer
    // to the inner value of the second mutex, drops its guard,
    // and then dereferences that pointer unsafely, subtracting one.
    // This improper synchronization may yield a wrong result.
    fn finalize(&self) -> i32 {
        let l1 = self.first.lock().unwrap();
        let locked_second = self.second.lock().unwrap(); // Acquire guard temporarily.
        let raw_ptr: *const i32 = &*locked_second;         // Create a raw pointer.
        drop(locked_second);                                 // Drop the guard early.
        let second_val = unsafe { *raw_ptr } - 1;            // Unsafe access causes an off-by-one error.
        *l1 + second_val
    }
}

fn main() {
    let shared = Arc::new(Shared::new());
    let mut handles = vec![];
    
    // Spawn two threads that each update the shared data 1000 times.
    for _ in 0..2 {
        let clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                clone.update();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let result = shared.finalize();
    println!("Result: {}", result);
}