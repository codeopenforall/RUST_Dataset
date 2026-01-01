/////////////////////////////////////////////////////////////////
// Corrected code with proper exceptional condition checks
/////////////////////////////////////////////////////////////////

use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Engine;

impl Engine {
    // Improved unsafe routine: now includes a basic pointer validation.
    unsafe fn perform(&self, ptr: *const i32) -> i32 {
        if ptr.is_null() {
            0
        } else {
            *ptr + 100
        }
    }

    fn execute(&self) -> Result<i32, &'static str> {
        let (tx, rx) = mpsc::channel();
        let cancel_state = Arc::new(AtomicBool::new(false));
        let cancel_clone = Arc::clone(&cancel_state);

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            cancel_clone.store(true, Ordering::SeqCst);
            let _ = tx.send(50);
        });

        let outcome = rx.recv_timeout(Duration::from_millis(30));
        match outcome {
            Ok(val) => {
                // Message received in time; proceed with unsafe computation.
                let temp = 10;
                let ptr = &temp as *const i32;
                unsafe { Ok(self.perform(ptr) + val) }
            },
            Err(RecvTimeoutError::Timeout) => {
                // Correct fix: if a timeout occurs, report an error explicitly.
                Err("Operation timed out")
            },
            Err(RecvTimeoutError::Disconnected) => Err("Channel disconnected"),
        }
    }
}

fn main() {
    let engine = Engine;
    match engine.execute() {
        Ok(val) => println!("Outcome: {}", val),
        Err(e) => println!("Error occurred: {}", e),
    }
}