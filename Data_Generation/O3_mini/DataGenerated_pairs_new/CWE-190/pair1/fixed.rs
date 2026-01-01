/* Corrected Example: This program fixes the integer overflow by using checked arithmetic.
   The update method returns a Result to indicate whether the addition succeeded without overflow.
   In the event of a potential overflow, the value remains unchanged and the error is handled.
   The overall structure (structs, methods, concurrency, main) mirrors a realistic codebase. */
use std::sync::{Arc, Mutex};
use std::thread;

struct Accumulator {
    value: u8,
}

impl Accumulator {
    fn new(init: u8) -> Self {
        Self { value: init }
    }

    // Fixed update using checked arithmetic.
    // If overflow would occur, it returns an error and leaves the value unchanged.
    fn update(&mut self, add: u8) -> Result<(), &'static str> {
        if let Some(new_val) = self.value.checked_add(add) {
            self.value = new_val;
            Ok(())
        } else {
            Err("integer overflow detected")
        }
    }

    fn get(&self) -> u8 {
        self.value
    }
}

// In this version, run_calculation returns the accumulator’s value.
// If an overflow is detected, the update is not applied.
fn run_calculation() -> u8 {
    let mut acc = Accumulator::new(250);
    // Attempt to update; if an overflow is detected, leave the value unchanged.
    let _ = acc.update(10).unwrap_or_else(|err| {
        // In a production environment, you might log or handle the error.
        // Here, we simply ignore the update to prevent corrupting the value.
        eprintln!("Warning: {}", err);
    });
    acc.get()
}

fn main() {
    let shared = Arc::new(Mutex::new(run_calculation()));
    let mut threads = vec![];
    for _ in 0..2 {
        let s = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            let val = *s.lock().unwrap();
            println!("Thread sees value: {}", val);
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    println!("Final result: {}", run_calculation());
}