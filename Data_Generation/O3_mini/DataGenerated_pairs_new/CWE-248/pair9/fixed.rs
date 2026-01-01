///////////////////////////////////////////////////////////
// In this corrected version the risky computation is wrapped
// in panic::catch_unwind so that any panic (such as from division
// by zero) is captured and converted into a controlled error.
// The thread’s result is subsequently checked and any panics are
// reported as error conditions rather than causing abnormal exit.
///////////////////////////////////////////////////////////

use std::sync::Arc;
use std::thread;
use std::panic;

pub struct Controller;

impl Controller {
    pub fn run(&self, input: i32) -> Result<i32, &'static str> {
        let shared = Arc::new(input);
        let cloned = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            // Capture potential panics during the computation.
            let res = panic::catch_unwind(|| {
                unsafe {
                    if *cloned == 0 {
                        // Instead of panicking, return an error.
                        Err("Division by zero")
                    } else {
                        Ok(100 / *cloned)
                    }
                }
            });
            match res {
                Ok(inner) => inner,
                // A panic was captured; return an error indicating thread panic.
                Err(_) => Err("Thread panicked")
            }
        });
        // Gracefully handle errors from joining the thread.
        match handle.join() {
            Ok(inner) => inner,
            Err(_) => Err("Thread join failed")
        }
    }
}

fn main() {
    let controller = Controller;
    // Even with an input of zero, the error is gracefully handled.
    match controller.run(0) {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Handled error: {}", err),
    }
}