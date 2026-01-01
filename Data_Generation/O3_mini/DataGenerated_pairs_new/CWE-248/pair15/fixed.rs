//////////////////////////
// This corrected implementation uses explicit error handling when joining threads.
// Instead of unwrapping the join result, it pattern matches on the outcome,
// returning a designated error code (-1) when a panic is detected.
// This practice prevents abnormal termination and ensures graceful error recovery.
//////////////////////////
use std::{thread, panic};

struct Engine;

impl Engine {
    fn run(&self, trigger: bool) -> i32 {
        // Spawn a thread running an unsafe closure.
        let handle = thread::spawn(move || {
            unsafe {
                if trigger {
                    panic!("panic in unsafe block caught later");
                }
                10
            }
        });
        // Properly handling the join result by matching its outcome.
        match handle.join() {
            Ok(val) => val,
            Err(_) => {
                // Return -1 to indicate that an error occurred instead of panicking.
                -1
            }
        }
    }
}

fn main() {
    let engine = Engine;
    // With the triggering condition active, this variant gracefully returns an error code.
    let result = engine.run(true);
    println!("Result: {}", result);
}