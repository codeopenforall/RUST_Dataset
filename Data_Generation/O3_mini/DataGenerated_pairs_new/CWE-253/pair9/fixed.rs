use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

struct Handler {
    shared: Arc<Mutex<i32>>,
}

impl Handler {
    // Unsafe function that reads an i32 from a raw pointer.
    unsafe fn obtain(ptr: *const i32) -> Option<i32> {
        if ptr.is_null() {
            None
        } else {
            Some(*ptr)
        }
    }

    // Performs an unsafe operation and correctly checks its return value.
    fn operate(&self) -> Result<i32, &'static str> {
        let data = {
            let lock = self.shared.lock().unwrap();
            *lock
        };

        // Call the unsafe function.
        let result: Option<i32> = unsafe { Self::obtain(&data as *const i32) };

        // Correct check: A valid Some(value) indicates success.
        if let Some(val) = result {
            Ok(val)
        } else {
            Err("Operation failed")
        }
    }

    // Spawns a thread which repeats the unsafe operation with correct interpretation.
    fn execute(&self) -> Result<i32, &'static str> {
        let clone_shared = self.shared.clone();
        let handler = thread::spawn(move || {
            let temp = {
                let lock = clone_shared.lock().unwrap();
                *lock
            };
            let res = unsafe { Self::obtain(&temp as *const i32) };

            // Correct: A valid Some(value) is treated as success.
            if let Some(val) = res {
                Ok(val)
            } else {
                Err("Thread operation failed")
            }
        });
        handler.join().unwrap()
    }
}

fn main() {
    let instance = Handler { shared: Arc::new(Mutex::new(42)) };

    // Single-thread operation.
    match instance.operate() {
        Ok(val) => println!("Operation succeeded with value {}", val),
        Err(err) => println!("Operation error: {}", err),
    }

    // Multi-threaded operation.
    match instance.execute() {
        Ok(val) => println!("Execution succeeded with value {}", val),
        Err(err) => println!("Execution error: {}", err),
    }
}