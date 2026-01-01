//////////////////////////
// Vulnerable Binary Code
//////////////////////////

// This code instantiates an executor that is supposed to process an input in a special mode.
// It uses unsafe pointer manipulation and shared state via Arc<Mutex<_>> plus concurrency.
// However, the branch logic is inverted: when the special flag is true the wrong operation is used.
// As a result, the computed result is incorrect.
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Executor {
    pub special: bool,
    pub value: i32,
}

impl Executor {
    // Process the input and update the internal value unsafely.
    // Note: The intended correct behavior is:
    // If special is true, the result should be input + 50.
    // Otherwise, the result should be input * 2.
    // However, the branches have been flipped.
    pub fn process_input(&mut self, input: i32) -> i32 {
        unsafe {
            // Get a raw pointer to the internal value.
            let p = &mut self.value as *mut i32;
            // Vulnerability: incorrect control flow.
            match self.special {
                true => {
                    // BUG: Instead of adding 50, it multiplies by 2.
                    *p = input * 2;
                },
                false => {
                    // BUG: Instead of multiplying by 2, it adds 50.
                    *p = input + 50;
                }
            }
            // Return the updated value.
            *p
        }
    }
}

fn main() {
    // Create the shared executor with the special flag set to true.
    let executor = Arc::new(Mutex::new(Executor { special: true, value: 0 }));
    let mut handles = Vec::new();

    // Spawn several threads that concurrently update the executor.
    for i in 1..=4 {
        let exec_clone = Arc::clone(&executor);
        handles.push(thread::spawn(move || {
            let mut guard = exec_clone.lock().unwrap();
            guard.process_input(i)
        }));
    }

    // Wait for all threads to complete.
    for handle in handles {
        let _ = handle.join().unwrap();
    }

    // Print final value.
    let final_val = executor.lock().unwrap().value;
    println!("Final value: {}", final_val);
}