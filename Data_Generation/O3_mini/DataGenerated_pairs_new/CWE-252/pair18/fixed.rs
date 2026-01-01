/* 
In this corrected version, the unsafe update function has been modified to avoid altering
the shared value when the update would violate the safe range [0, 100]. In addition,
the caller explicitly checks the update result and skips the update operation if an error is returned.
This ensures the system remains in a secure state.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct SecureData {
    value: i32,
}

impl SecureData {
    // Corrected update: if the change is out-of-range, do not update the value.
    unsafe fn update(&mut self, delta: i32) -> Result<(), &'static str> {
        if self.value + delta < 0 || self.value + delta > 100 {
            return Err("value out of range");
        }
        self.value += delta;
        Ok(())
    }
}

// Run multiple concurrent updates while checking the result of each update.
fn process(data: Arc<Mutex<SecureData>>, delta: i32) {
    let mut handles = Vec::new();
    for _ in 0..5 {
        let data_cloned = Arc::clone(&data);
        let h = thread::spawn(move || {
            unsafe {
                let mut guard = data_cloned.lock().unwrap();
                // Fix: explicitly check the result; if update fails, skip without modifying the state.
                if guard.update(delta).is_err() {
                    // Optionally, log the error or take remedial action.
                    return;
                }
            }
        });
        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }
}

// Exposed function to drive the computation; used for main and testing.
pub fn run() -> i32 {
    let initial = SecureData { value: 50 };
    let shared = Arc::new(Mutex::new(initial));

    // A safe update: adding 10 five times results in 100.
    process(Arc::clone(&shared), 10);
    // An unsafe update is attempted. With the fix, when the update would go out-of-range,
    // the update is not applied and the error is properly handled.
    process(Arc::clone(&shared), 60);

    // Return the final state, which remains safe.
    let final_val = shared.lock().unwrap().value;
    final_val
}

fn main() {
    let final_value = run();
    println!("Final value: {}", final_value);
}