//////////////////////
// Vulnerable Rust Code
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
// The vulnerability is in the check of the return value from try_update: an error result is misinterpreted as a success.
fn concurrent_update(shared: Arc<Mutex<Container>>, new_number: i32) -> bool {
    let mut threads = vec![];
    for _ in 0..4 {
        let data = shared.clone();
        threads.push(thread::spawn(move || {
            unsafe {
                let mut guard = data.lock().unwrap();
                let ret = guard.try_update(new_number);
                // Vulnerability: Incorrect check of the function return value.
                // Instead of checking for Ok(_), it treats Err(_) as a successful update.
                if ret.is_err() {
                    true
                } else {
                    false
                }
            }
        }));
    }
    let mut success = false;
    for handle in threads {
        // Combine thread results; if any thread erroneously signals success, we propagate that.
        success |= handle.join().unwrap();
    }
    success
}

fn main() {
    let shared = Arc::new(Mutex::new(Container { number: 42 }));
    // Triggering input that should lead to a failure update (0 is invalid).
    // However, due to the vulnerability, the error is misinterpreted as success.
    if concurrent_update(shared.clone(), 0) {
        println!("Concurrent update reported success (vulnerable behavior).");
    } else {
        println!("Concurrent update reported failure.");
    }
}