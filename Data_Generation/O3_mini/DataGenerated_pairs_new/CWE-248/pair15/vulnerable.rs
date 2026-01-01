//////////////////////////
// A real-world style code snippet utilizing threads and unsafe blocks.
// This implementation spawns a worker thread that executes an unsafe block.
// When provided with a triggering condition, the unsafe block panics,
// and the thread’s panic is not caught when joining, leading to abnormal termination.
//////////////////////////
use std::{thread, panic};

struct Engine;

impl Engine {
    fn run(&self, trigger: bool) -> i32 {
        // Spawn a thread running an unsafe closure.
        let handle = thread::spawn(move || {
            unsafe {
                // Simulate an execution error via a panic within an unsafe context.
                if trigger {
                    panic!("uncaught panic in unsafe block");
                }
                // Normal computation result.
                10
            }
        });
        // Vulnerability: directly unwrapping the thread join result.
        // If the thread panics, unwrap() propagates the panic, causing abnormal termination.
        handle.join().unwrap()
    }
}

fn main() {
    let engine = Engine;
    // This call, using a triggering condition, will produce an uncaught panic.
    let result = engine.run(true);
    println!("Result: {}", result);
}