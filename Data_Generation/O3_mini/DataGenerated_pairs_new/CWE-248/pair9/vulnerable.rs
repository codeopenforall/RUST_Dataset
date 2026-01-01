///////////////////////////////////////////////////////////
// This Rust program demonstrates an unsafe, concurrent
// execution that does not catch panics arising from a
// division-by-zero situation. In a threaded context, the
// unsafe block is used to simulate an FFI boundary. When
// an input of zero is provided, the division in the unsafe
// block panics. The thread’s panic is not caught, and the
// subsequent join().unwrap() triggers an uncaught panic,
// mimicking CWE-248 due to abnormal termination by a panic.
///////////////////////////////////////////////////////////

use std::sync::Arc;
use std::thread;

pub struct Controller;

impl Controller {
    pub fn run(&self, input: i32) -> Result<i32, &'static str> {
        let shared = Arc::new(input);
        let cloned = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            unsafe {
                if *cloned == 0 {
                    // Vulnerability: division by zero will panic 
                    // and is not caught, propagating the panic
                    100 / *cloned
                } else {
                    100 / *cloned
                }
            }
        });
        // Vulnerability: join is unwrapped; if the thread panicked,
        // this unwrap will re-propagate the panic instead of handling it.
        let result = handle.join().unwrap();
        Ok(result)
    }
}

fn main() {
    let controller = Controller;
    // Trigger the vulnerability with an input that causes panic.
    match controller.run(0) {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    }
}