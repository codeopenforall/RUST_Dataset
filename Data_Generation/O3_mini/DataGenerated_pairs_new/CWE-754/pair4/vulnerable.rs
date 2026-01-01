//////////////////////////////
// Vulnerable Code (Unsafe and Concurrency Flaw)
// CWE-754: Improper Check for Exceptional Conditions
// This version incorrectly handles timeout errors by treating them as successful operations.
//////////////////////////////

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn process(input: u32) -> Result<u32, &'static str> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        // If input equals 1, simulate a delay to force a timeout.
        if input == 1 {
            thread::sleep(Duration::from_millis(100));
        }
        // Unsafe block to mimic raw pointer usage in real-world low-level operations.
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
            // Vulnerability: Instead of treating timeout as error,
            // it is improperly handled as a successful operation returning 0.
            if let mpsc::RecvTimeoutError::Timeout = e {
                Ok(0)
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