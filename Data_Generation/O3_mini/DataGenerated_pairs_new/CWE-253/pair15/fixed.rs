//////////////////////
// Fixed Rust Code
//////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Container {
    number: i32,
}

impl Container {
    // Unsafe update method that rejects zero as an invalid update.
    unsafe fn try_update(&mut self, new_number: i32) -> Result<(), &'static str> {
        if new_number == 0 {
            return Err("Zero is not allowed");
        }
        self.number = new_number;
        Ok(())
    }
}

// This function spawns multiple threads that concurrently attempt the update.
// The fix is to correctly interpret the return value from try_update.
fn concurrent_update(shared: Arc<Mutex<Container>>, new_number: i32) -> bool {
    let mut threads = vec![];
    for _ in 0..4 {
        let data = shared.clone();
        threads.push(thread::spawn(move || {
            unsafe {
                let mut guard = data.lock().unwrap();
                let ret = guard.try_update(new_number);
                // Fixed: Correctly check for a successful update.
                if ret.is_ok() {
                    true
                } else {
                    false
                }
            }
        }));
    }
    let mut success = false;
    for handle in threads {
        // Combine thread results; a thread reports success only if the update truly succeeded.
        success |= handle.join().unwrap();
    }
    success
}

fn main() {
    let shared = Arc::new(Mutex::new(Container { number: 42 }));
    // Triggering input that should lead to a failure update (0 is invalid).
    // The updated logic correctly reports failure.
    if concurrent_update(shared.clone(), 0) {
        println!("Concurrent update reported success.");
    } else {
        println!("Concurrent update reported failure (fixed behavior).");
    }
}