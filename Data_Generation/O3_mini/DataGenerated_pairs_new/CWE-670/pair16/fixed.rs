//////////////////////////
// Corrected Code Begin
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
            let mut data = state_clone.lock().unwrap();
            unsafe {
                // Correct control flow: only when code equals 42 the state is updated to 99.
                match code {
                    42 => {
                        *data = 99;
                        return Ok(());
                    },
                    _ => {
                        return Err("invalid code");
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
    let result = app.update_state(42);
    match result {
        Ok(()) => println!("Success, state is {}", app.read_state()),
        Err(e) => println!("Error: {}", e),
    }
}

////////////////////////
// Corrected Code End
////////////////////////