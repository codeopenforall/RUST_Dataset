/////////////////////////////////////////////////////////////
// Vulnerable Code: Improper check for exceptional conditions
// This code spawns a worker thread that will take too long to send
// its result back through a channel. The main routine tries to
// get the result with a short timeout. However, when a timeout
// occurs the error is misinterpreted inside an unsafe block,
// causing the function to return success. This flaw can expose
// the application to logic errors and potential security issues.
/////////////////////////////////////////////////////////////

use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

struct Operation;

impl Operation {
    // Executes an operation that is expected to complete within a short timeout.
    // In the case of a timeout error, the vulnerable code erroneously treats it as success.
    pub fn run() -> Result<(), &'static str> {
        // create a channel for communication
        let (tx, rx) = channel();

        // Spawn a worker thread that intentionally sleeps longer than the timeout.
        thread::spawn(move || {
            // Simulate heavy computation or blocking I/O.
            thread::sleep(Duration::from_millis(100));
            // Use an unsafe block to simulate low-level memory manipulation (not needed logically, but mimics real-world unsafe code usage).
            unsafe {
                // Unsafe pointer manipulation as a dummy operation.
                let bogus: *mut u32 = std::ptr::null_mut();
                let _ = bogus; 
            }
            let _ = tx.send(42); // Send a dummy result.
        });

        // Set a short timeout that will trigger before the worker finishes.
        let timeout = Duration::from_millis(10);
        let start = Instant::now();
        let result = rx.recv_timeout(timeout);

        // Improper error handling: the timeout error is misinterpreted as a success.
        match result {
            Ok(val) => {
                // Normally return success for a valid result.
                if val == 42 {
                    Ok(())
                } else {
                    Err("Unexpected result")
                }
            }
            Err(e) => {
                unsafe {
                    // Vulnerability: Wrongly treating Timeout as an acceptable outcome.
                    if std::mem::discriminant(&e) == std::mem::discriminant(&RecvTimeoutError::Timeout) {
                        // Here, instead of propagating the timeout error, it returns Ok.
                        Ok(())
                    } else {
                        Err("Channel disconnected unexpectedly")
                    }
                }
            }
        }
    }
}

fn main() {
    match Operation::run() {
        Ok(_) => println!("Operation succeeded (vulnerable behavior)!"),
        Err(err) => println!("Operation failed: {}", err),
    }
}