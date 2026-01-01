//////////////////////////
// Vulnerable Code Sample
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

    // This function performs an unsafe memory update without proper bounds checks.
    // It lets an attacker influence the memory state in an uncontrolled way.
    pub fn update(&self, index: usize, value: i32) {
        let mut lock = self.state.lock().unwrap();
        unsafe {
            // Unsafe pointer arithmetic without proper bounds validation.
            let ptr = lock.as_mut_ptr();
            *ptr.add(index) = value;
        }
    }

    // Function that verifies an invariant on the shared state.
    // The assertion is reachable if the shared state is attacker-influenced.
    pub fn verify(&self) {
        let lock = self.state.lock().unwrap();
        // If the attacker has updated index 0 to a negative value, this assertion fires.
        assert!(lock[0] >= 0, "Invariant violated: state[0] is negative");
    }
}

// The application entry point function used by external test oracles.
pub fn start() {
    let ctx = Context::new(10);

    // Spawn an attacker-controlled thread that unsafely writes a negative value.
    let handle = {
        let shared = ctx.state.clone();
        thread::spawn(move || {
            let mut data = shared.lock().unwrap();
            unsafe {
                // Unsafe write that manipulates memory directly.
                let ptr = data.as_mut_ptr();
                // Attacker forces a negative value into the invariant index.
                *ptr.add(0) = -999;
            }
        })
    };

    handle.join().unwrap();

    // Optionally, another update call can be made by trusted code:
    // Here commented out because it would overwrite the malicious change.
    // ctx.update(0, 1);

    // Reachable assertion: if state[0] is negative, program panics.
    ctx.verify();

    println!("Execution completed in the vulnerable configuration.");
}

fn main() {
    start();
}