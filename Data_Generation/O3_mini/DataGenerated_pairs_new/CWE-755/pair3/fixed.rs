/*
 This revised example corrects the earlier flawed error handling.
 Instead of suppressing errors, it propagates them to the caller using the '?' operator.
 The program maintains similar unsafe operations and concurrency, but it properly aborts when exceptional conditions occur.
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

// Function to simulate input processing, which fails if input is empty.
pub fn analyze(input: &str) -> Result<u32, &'static str> {
    if input.is_empty() {
        Err("Empty input not allowed")
    } else {
        Ok(input.len() as u32)
    }
}

// Main application function that properly propagates errors.
pub fn run_app(input: &str) -> Result<(), &'static str> {
    // Instead of swallowing the error, propagate it if the input is empty.
    let factor = analyze(input)?;

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
    // The provided input is deliberately empty.
    // The error from run_app is correctly propagated and causes termination.
    if let Err(e) = run_app("") {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}