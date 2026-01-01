//////////////////////////
// Fixed Binary Code
//////////////////////////

// This corrected code instantiates an executor that processes an input in a special mode correctly.
// It uses unsafe pointer manipulation and shared state via Arc<Mutex<_>> plus concurrency,
// but the branch logic has been fixed to properly distinguish the modes.
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Executor {
    pub special: bool,
    pub value: i32,
}

impl Executor {
    // Process the input and update the internal value unsafely.
    // Correct behavior:
    // If special is true, then add 50 to the input.
    // Otherwise, multiply the input by 2.
    pub fn process_input(&mut self, input: i32) -> i32 {
        unsafe {
            let p = &mut self.value as *mut i32;
            match self.special {
                true => {
                    // Correct special mode: add 50.
                    *p = input + 50;
                },
                false => {
                    // Normal mode: multiply by 2.
                    *p = input * 2;
                }
            }
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