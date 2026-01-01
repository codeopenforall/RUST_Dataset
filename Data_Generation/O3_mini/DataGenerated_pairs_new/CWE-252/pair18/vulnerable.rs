/* 
This code simulates a security‐critical update on shared data using concurrency,
unsafe blocks, and a custom update method. The update method is flawed: if an update
would cause the value to go out of the allowed range [0, 100], it still applies the update
before returning an error. The caller ignores the error result. This unchecked error handling
can result in an insecure final state.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct SecureData {
    value: i32,
}

impl SecureData {
    // Unsafe update: returns an error if out-of-range but still applies the change.
    unsafe fn update(&mut self, delta: i32) -> Result<(), &'static str> {
        if self.value + delta < 0 || self.value + delta > 100 {
            // Flawed behavior: even though the result indicates an error, the update is applied.
            self.value += delta;
            return Err("value out of range");
        }
        self.value += delta;
        Ok(())
    }
}

// Run multiple concurrent updates without checking the result.
fn process(data: Arc<Mutex<SecureData>>, delta: i32) {
    let mut handles = Vec::new();
    for _ in 0..5 {
        let data_cloned = Arc::clone(&data);
        let h = thread::spawn(move || {
            unsafe {
                let mut guard = data_cloned.lock().unwrap();
                // Vulnerability: the error returned by update is ignored.
                let _ = guard.update(delta);
            }
        });
        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }
}

// Exposed function to drive the computation; used for both main and testing.
pub fn run() -> i32 {
    // Start with a safe value.
    let initial = SecureData { value: 50 };
    let shared = Arc::new(Mutex::new(initial));

    // This update is within bounds; adding 10 five times brings the value to 100.
    process(Arc::clone(&shared), 10);
    // This update should be disallowed. However, the unsafe update erroneously applies the change.
    process(Arc::clone(&shared), 60);

    // Return the final state.
    let final_val = shared.lock().unwrap().value;
    final_val
}

fn main() {
    let final_value = run();
    println!("Final value: {}", final_value);
}