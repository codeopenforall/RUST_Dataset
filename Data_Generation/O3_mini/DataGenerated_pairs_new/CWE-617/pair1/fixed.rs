////////////////////////////////////////////////////////////////////////////////////////////////////
// The revised service performs explicit bounds checking on the multiplier input before updating
// the internal counter, thereby avoiding the reachable assertion vulnerability. Rather than
// asserting the safe state (which can be externally controlled), the method now returns a Result
// indicating if the update failed due to an unsafe multiplier value. This design prevents a DoS
// by not unconditionally panicking and instead handling the error gracefully.
////////////////////////////////////////////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

const THRESHOLD: usize = 100;

struct Service {
    value: usize,
}

impl Service {
    fn new() -> Self {
        Service { value: 1 }
    }

    // The update method now returns a Result. It verifies that applying the multiplier
    // will not put the internal value beyond a safe threshold. Only if the check passes does it
    // use unsafe block to update the counter.
    fn update(&mut self, multiplier: usize) -> Result<(), &'static str> {
        let new_value = self.value.wrapping_mul(multiplier);
        if new_value >= THRESHOLD {
            // Return an error instead of triggering an assertion.
            return Err("Update rejected: value would exceed safe threshold.");
        }
        let ptr: *mut usize = &mut self.value;
        unsafe {
            *ptr = new_value;
        }
        Ok(())
    }
}

fn main() {
    let shared = Arc::new(Mutex::new(Service::new()));
    let mut threads = vec![];

    // Spawn threads simulating concurrent updates.
    for i in 1..=5 {
        let service_clone = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            let factor = if i == 3 { 150 } else { 2 };
            let mut instance = service_clone.lock().unwrap();
            // Instead of a panic-inducing assert, we handle the error gracefully.
            if let Err(e) = instance.update(factor) {
                eprintln!("Thread {}: {}", i, e);
            }
        }));
    }

    for handle in threads {
        let _ = handle.join();
    }
    println!("Final value: {}", shared.lock().unwrap().value);
}