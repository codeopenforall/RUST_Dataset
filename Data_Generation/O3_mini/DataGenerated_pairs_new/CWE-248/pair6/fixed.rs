//////////////////////////////////////////////////////////////
// This revised code wraps the thread join in proper error
// handling. Instead of unwrapping the join result (which can
// propagate a panic), the join result is matched to capture any
// panics and convert them to an error result. This prevents the
// panic from propagating and properly handles the exceptional case.
//////////////////////////////////////////////////////////////
use std::thread;

pub struct Manager;

impl Manager {
    pub fn process(&self, value: i32) -> Result<i32, &'static str> {
        let handle = thread::spawn(move || -> i32 {
            unsafe {
                let ptr = &value as *const i32;
                let temp = *ptr;
                if temp == 42 {
                    panic!("Triggered panic in unsafe block");
                }
                temp * 2
            }
        });
        // Instead of unwrap(), match on the join result to catch panics.
        match handle.join() {
            Ok(result) => Ok(result),
            Err(_) => Err("Operation failed"),
        }
    }
}

fn main() {
    let manager = Manager;
    // The trigger value now is safely handled.
    match manager.process(42) {
        Ok(val) => println!("Completed execution with result: {}", val),
        Err(e) => println!("Handled error: {}", e),
    }
}