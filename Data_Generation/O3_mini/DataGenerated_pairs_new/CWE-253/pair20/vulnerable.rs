//////////////////////////////////////////////
// Vulnerable Code for CWE-253 Sample
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

trait Task {
    fn execute(&self, value: i32);
}

struct Engine {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Engine {
    fn new() -> Self {
        Engine {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // This function is intended to update a shared vector only if the value is valid (non-negative).
    // However, the check is reversed: it returns Ok even for an invalid (negative) value.
    fn perform_update(&self, value: i32) -> Result<(), String> {
        // Simulate an unsafe operation: a meaningless pointer write used to mimic low-level code.
        unsafe {
            let dummy: i32 = 0;
            // Create a mutable pointer from a constant reference (not really modifying shared state)
            let ptr = &dummy as *const i32 as *mut i32;
            // Volatile write to simulate unsafe access
            ptr.write_volatile(42);
        }
        // Vulnerability: The function returns Ok even when value is negative,
        // and returns an Err for values that are actually valid.
        if value < 0 {
            return Ok(());
        } else {
            return Err("Unexpected positive value".to_string());
        }
    }

    // Spawns several threads to invoke the update concurrently.
    // Each thread calls perform_update and then, regardless of the result, pushes the value into the shared vector.
    fn run_tasks(&self, value: i32) {
        let mut handles = Vec::new();
        for _ in 0..4 {
            let data_clone = Arc::clone(&self.data);
            let eng = self.clone();
            handles.push(thread::spawn(move || {
                match eng.perform_update(value) {
                    Ok(()) => {
                        let mut vec_lock = data_clone.lock().unwrap();
                        vec_lock.push(value);
                    }
                    Err(_e) => {
                        // Even in the error branch the value is appended.
                        let mut vec_lock = data_clone.lock().unwrap();
                        vec_lock.push(value);
                    }
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    // Retrieve a copy of the vector for validation.
    fn get_data(&self) -> Vec<i32> {
        self.data.lock().unwrap().clone()
    }
}

// Manually implement Clone for Engine so it can be moved into threads.
impl Clone for Engine {
    fn clone(&self) -> Self {
        Engine {
            data: Arc::clone(&self.data),
        }
    }
}

// Provide a dummy trait implementation.
impl Task for Engine {
    fn execute(&self, value: i32) {
        self.run_tasks(value);
    }
}

fn main() {
    let engine = Engine::new();
    // Negative value should be rejected.
    // The vulnerability inverts the check so that -1 is accepted.
    engine.execute(-1);
    let result = engine.get_data();
    println!("Data: {:?}", result);
}