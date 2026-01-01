//////////////////////////
// Vulnerable Code Begin
//////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Coordinator {
    state: Arc<Mutex<u32>>,
}

impl Coordinator {
    fn new() -> Self {
        Coordinator {
            state: Arc::new(Mutex::new(0)),
        }
    }

    fn update_state(&self, code: u32) -> Result<(), &'static str> {
        let state_clone = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            // Acquire the lock and enter an unsafe block to perform a pointer-based update.
            let mut data = state_clone.lock().unwrap();
            unsafe {
                // Flawed control logic: even when code equals 42 (a valid trigger),
                // the match branch always returns an error without updating the state.
                match code {
                    42 => { // Intended: update data to 99.
                        // BUG: Incorrect control flow that prevents the proper update.
                        return Err("operation failed");
                    },
                    _ => {
                        *data = 0;
                        return Err("operation failed");
                    },
                }
            }
        });
        handle.join().unwrap()
    }

    fn read_state(&self) -> u32 {
        *self.state.lock().unwrap()
    }
}

fn main() {
    let app = Coordinator::new();
    // For valid trigger code 42, the caller expects the state to be set to 99.
    let result = app.update_state(42);
    match result {
        Ok(()) => println!("Success, state is {}", app.read_state()),
        Err(e) => println!("Error: {}", e),
    }
}

////////////////////////
// Vulnerable Code End
////////////////////////