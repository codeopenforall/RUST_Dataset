/*
 This example demonstrates a flawed handling of exceptional conditions.
 The program defines a shared processor that uses unsafe operations and spawns threads.
 An input is processed with a function that may return an error, but the error is silently
 replaced by a default value (using unwrap_or_default), leading to incorrect behavior.
*/
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Core {
    pub count: u32,
}

impl Core {
    // Unsafe operation that modifies the internal state.
    pub unsafe fn update(&mut self, add: u32) -> u32 {
        self.count = self.count.wrapping_add(1);
        self.count.wrapping_add(add)
    }
}

// Function to simulate input processing, which may fail if input is empty.
pub fn analyze(input: &str) -> Result<u32, &'static str> {
    if input.is_empty() {
        Err("Empty input not allowed")
    } else {
        Ok(input.len() as u32)
    }
}

// Main application function that swallows errors using unwrap_or_default.
pub fn run_app(input: &str) -> Result<(), &'static str> {
    // The error from analyze() is suppressed, returning the default value 0.
    let factor = analyze(input).unwrap_or_default();  // Vulnerability occurs here

    let core_state = Arc::new(Mutex::new(Core { count: 0 }));
    let mut workers = vec![];

    // Spawn several threads that perform unsafe updates to the state.
    for _ in 0..5 {
        let state = Arc::clone(&core_state);
        let f = factor;
        let handle = thread::spawn(move || unsafe {
            let mut lock = state.lock().unwrap();
            lock.update(f)
        });
        workers.push(handle);
    }

    // Join all threads.
    for worker in workers {
        let res = worker.join().unwrap();
        println!("Worker result: {}", res);
    }
    Ok(())
}

fn main() {
    // The provided input is deliberately empty; the error is incorrectly suppressed.
    if let Err(e) = run_app("") {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}