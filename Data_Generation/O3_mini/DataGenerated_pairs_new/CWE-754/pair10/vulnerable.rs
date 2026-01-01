/////////////////////////////////////////////////////////////////
// Vulnerability demonstration code for exceptional condition check
/////////////////////////////////////////////////////////////////

use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Engine;

impl Engine {
    // Unsafe routine that performs a raw pointer dereference.
    // (Note: In realistic code, the pointer may point to sensitive data.)
    unsafe fn perform(&self, ptr: *const i32) -> i32 {
        // Vulnerability: no null check or validation; dangerous dereference.
        *ptr + 100
    }

    fn execute(&self) -> Result<i32, &'static str> {
        let (tx, rx) = mpsc::channel();
        let cancel_state = Arc::new(AtomicBool::new(false));
        let cancel_clone = Arc::clone(&cancel_state);

        // Spawn a worker thread that simulates a delayed operation.
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100)); // Delay that exceeds our timeout.
            cancel_clone.store(true, Ordering::SeqCst); // Set the cancellation flag.
            let _ = tx.send(50);
        });

        // Attempt to receive a value with a short timeout period.
        let outcome = rx.recv_timeout(Duration::from_millis(30));
        match outcome {
            Ok(val) => {
                // Received a message in time; perform unsafe computation.
                let temp = 10;
                let ptr = &temp as *const i32;
                unsafe { Ok(self.perform(ptr) + val) }
            },
            Err(RecvTimeoutError::Timeout) => {
                // <-- Vulnerability: Improper exceptional condition check.
                // The timeout case is mishandled: cancellation state is checked but
                // regardless of its value, a successful result with a default value is returned.
                if cancel_state.load(Ordering::SeqCst) {
                    // Erroneously treating a timed-out and then canceled operation as successful.
                    Ok(0)
                } else {
                    Ok(0)
                }
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