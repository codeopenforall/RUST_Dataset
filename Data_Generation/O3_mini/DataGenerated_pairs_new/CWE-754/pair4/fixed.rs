//////////////////////////////
// Fixed Code (Corrected Exception Check)
// CWE-754 Fix: Properly treat timeout conditions as errors.
//////////////////////////////

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn process(input: u32) -> Result<u32, &'static str> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        // For input equals 1, simulate a delay to force a timeout.
        if input == 1 {
            thread::sleep(Duration::from_millis(100));
        }
        // Unsafe usage preserved to mimic realistic low-level operations.
        unsafe {
            let ptr = &input as *const u32;
            let value = *ptr;
            let _ = tx.send(value);
        }
    });
    // Attempt to receive a value with a timeout.
    match rx.recv_timeout(Duration::from_millis(50)) {
        Ok(v) => Ok(v),
        Err(e) => {
            // Fix: Instead of misinterpreting a timeout as success,
            // the timeout error is now correctly handled by propagating an error.
            if let mpsc::RecvTimeoutError::Timeout = e {
                Err("Operation timed out")
            } else {
                Err("Channel disconnected")
            }
        }
    }
}

fn main() {
    // Demonstration call with an input expected to succeed.
    match process(2) {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}