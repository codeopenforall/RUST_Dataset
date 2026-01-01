///////////////////////////////////////////////////////////////
// Vulnerable implementation: Improper Check for Exceptional Conditions
// This implementation erroneously recovers from a timeout error 
// by converting it into a successful result using unsafe pointer access.
///////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq)]
enum ExecError {
    Timeout,
    Cancel,
    Internal,
}

struct Manager;

impl Manager {
    fn execute(&self, duration: Duration) -> Result<u32, ExecError> {
        let (tx, rx) = channel();
        let shared: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

        let shared_clone = Arc::clone(&shared);
        thread::spawn(move || {
            // Simulate a condition based on the duration input.
            if duration < Duration::from_millis(50) {
                unsafe {
                    // Unsafe block simulating direct pointer manipulation.
                    let ptr: *mut u32 = &mut *shared_clone.lock().unwrap();
                    *ptr = 0;
                }
                let _ = tx.send(Err(ExecError::Timeout));
            } else {
                unsafe {
                    let ptr: *mut u32 = &mut *shared_clone.lock().unwrap();
                    *ptr = 42;
                }
                let _ = tx.send(Ok(42));
            }
        });

        // Wait up to 100ms for the result.
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(val)) => Ok(val),
            Ok(Err(e)) => {
                // Vulnerability: improper handling of timeout exceptional condition.
                // Instead of propagating the error, it erroneously recovers.
                if let ExecError::Timeout = e {
                    let val = *shared.lock().unwrap();
                    return Ok(val);
                }
                Err(e)
            },
            Err(RecvTimeoutError::Timeout) => Err(ExecError::Timeout),
            Err(_) => Err(ExecError::Internal),
        }
    }
}

fn main() {
    let manager = Manager;
    // Input that triggers a timeout scenario.
    match manager.execute(Duration::from_millis(30)) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {:?}", e),
    }
}