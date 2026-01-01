///////////////////////////////////////////////
// Vulnerable Version - Unrestricted Resource Consumption
///////////////////////////////////////////////

use std::env;
use std::thread;
use std::sync::mpsc::{channel, Sender};

const SIZE: usize = 10 * 1024 * 1024; // 10 MB allocated per job

struct Handler {
    sender: Sender<Vec<u8>>,
}

impl Handler {
    fn new(sender: Sender<Vec<u8>>) -> Self {
        Handler { sender }
    }

    fn run(&self) {
        let sender_copy = self.sender.clone();
        thread::spawn(move || {
            // Dangerous: unbounded allocation without any validation.
            let mut buffer = Vec::with_capacity(SIZE);
            unsafe {
                // Unsafely setting length without initialization.
                buffer.set_len(SIZE);
            }
            // Send the allocated large buffer.
            sender_copy.send(buffer).unwrap();
        });
    }
}

// This function spawns a number of threads without imposing any limits.
// There is no backpressure or resource limit check.
fn execute_jobs(count: usize) -> () {
    let (tx, rx) = channel();
    let handler = Handler::new(tx);
    for _ in 0..count {
        handler.run();
    }
    // Collect results (this may accumulate large amounts of memory).
    for _ in 0..count {
        let _ = rx.recv().unwrap();
    }
}

// Exposed function for testing purposes.
#[no_mangle]
pub fn run_app(jobs: usize) -> Result<(), &'static str> {
    // No restrictions on jobs: the caller may trigger uncontrolled resource use.
    execute_jobs(jobs);
    Ok(())
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let num_jobs: usize = if arguments.len() > 1 {
        arguments[1].parse().unwrap_or(0)
    } else {
        0
    };
    // NOTE: Missing any check on the number of jobs.
    run_app(num_jobs).unwrap();
    println!("Processing complete");
}