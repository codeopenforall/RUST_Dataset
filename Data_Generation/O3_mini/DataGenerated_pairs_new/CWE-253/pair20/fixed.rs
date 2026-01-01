//////////////////////////////////////////////
// Corrected Code for CWE-253 Sample
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

    // Correctly assess the input: only non-negative values return Ok.
    // Negative inputs cause the function to return an error and halt further updates.
    fn perform_update(&self, value: i32) -> Result<(), String> {
        // Simulate unsafe operations to mimic low-level processing.
        unsafe {
            let dummy: i32 = 0;
            let ptr = &dummy as *const i32 as *mut i32;
            ptr.write_volatile(42);
        }
        if value < 0 {
            return Err("Negative value not allowed".to_string());
        } else {
            return Ok(());
        }
    }

    // Uses concurrent threads to update shared state.
    // In this corrected version, only valid values (non-negative) lead the threads to update the shared vector.
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
                    Err(e) => {
                        // In case of error the update is skipped.
                        eprintln!("Error: {}", e);
                    }
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    // Accessor method to retrieve internal state for verification.
    fn get_data(&self) -> Vec<i32> {
        self.data.lock().unwrap().clone()
    }
}

// Manual Clone implementation to allow Engine to be transferred into threads.
impl Clone for Engine {
    fn clone(&self) -> Self {
        Engine {
            data: Arc::clone(&self.data),
        }
    }
}

// Dummy trait implementation.
impl Task for Engine {
    fn execute(&self, value: i32) {
        self.run_tasks(value);
    }
}

fn main() {
    let engine = Engine::new();
    // With the corrected code, the negative input (-1) will be rejected.
    engine.execute(-1);
    let result = engine.get_data();
    println!("Data: {:?}", result);
}