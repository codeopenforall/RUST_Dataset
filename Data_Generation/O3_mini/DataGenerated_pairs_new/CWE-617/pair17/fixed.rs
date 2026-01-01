//////////////////////////
// Fixed Code Sample
//////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

pub struct Context {
    pub state: Arc<Mutex<Vec<i32>>>,
}

impl Context {
    pub fn new(size: usize) -> Self {
        Context {
            state: Arc::new(Mutex::new(vec![0; size])),
        }
    }

    // Updated function using safe Rust APIs with explicit bounds checking.
    pub fn update(&self, index: usize, value: i32) -> Result<(), &'static str> {
        let mut lock = self.state.lock().unwrap();
        if index < lock.len() {
            lock[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    // Verifies that the shared state meets its invariant.
    pub fn verify(&self) -> Result<(), &'static str> {
        let lock = self.state.lock().unwrap();
        if lock[0] >= 0 {
            Ok(())
        } else {
            Err("Invariant violated: state[0] is negative")
        }
    }
}

// The application entry point function used by external test oracles.
pub fn start() -> Result<(), &'static str> {
    let ctx = Context::new(10);

    // Spawn a thread that performs a safe update.
    let handle = {
        let shared = ctx.state.clone();
        thread::spawn(move || {
            let mut data = shared.lock().unwrap();
            // Using safe indexing to update the shared state.
            if !data.is_empty() {
                data[0] = 42; // Deterministically set a valid positive value.
            }
        })
    };

    handle.join().unwrap();

    // Perform an additional safe update on a different index.
    ctx.update(5, 100)?;
    // Verify that the invariant holds.
    ctx.verify()?;
    println!("Execution completed in the fixed configuration.");
    Ok(())
}

fn main() {
    start().unwrap();
}