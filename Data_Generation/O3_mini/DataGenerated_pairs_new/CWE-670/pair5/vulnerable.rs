//////////////// Vulnerable Code Begin
use std::sync::{Arc, Mutex};
use std::thread;

struct Controller {
    counter: Arc<Mutex<i32>>,
}

impl Controller {
    fn new(initial: i32) -> Self {
        Controller {
            counter: Arc::new(Mutex::new(initial)),
        }
    }

    // This function updates the counter concurrently.
    // It uses an unsafe block to perform arithmetic with potential wrapping.
    // The control flow checking the result is flawed.
    fn update(&self, adjustment: i32) -> Result<i32, &'static str> {
        let shared = Arc::clone(&self.counter);
        let handle = thread::spawn(move || {
            let mut num = shared.lock().unwrap();
            unsafe {
                // Use wrapping_add to simulate low-level arithmetic; intentionally using unsafe block.
                *num = num.wrapping_add(adjustment);
            }
        });
        handle.join().unwrap();

        // Read the updated value.
        let current = *self.counter.lock().unwrap();

        // Flawed control flow:
        // The intended logic is to return Ok(current) for non-negative values,
        // but the condition is inverted here.
        if current >= 0 {
            Err("Always incorrect branch taken")
        } else {
            Ok(current)
        }
    }
}

fn main() {
    let controller = Controller::new(1);
    // For a positive adjustment, the expected correct behavior is to succeed,
    // but due to the control flow bug, it will return an error.
    match controller.update(10) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
//////////////// Vulnerable Code End