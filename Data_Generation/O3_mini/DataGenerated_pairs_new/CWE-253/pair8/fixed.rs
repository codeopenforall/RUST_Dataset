use std::sync::{Arc, Mutex};
use std::thread;

struct Controller {
    total: i32,
}

impl Controller {
    // This unsafe function returns 0 on success and a nonzero error code when the operation cannot be applied.
    unsafe fn unsafe_adjustment(&mut self, increment: i32) -> i32 {
        if self.total == 0 {
            1 // error code if the current state is invalid
        } else {
            self.total += increment;
            0 // success code
        }
    }

    // Fixed: The return value from the unsafe call is now correctly interpreted.
    fn adjust(&mut self, increment: i32) -> Result<(), &'static str> {
        unsafe {
            let res = self.unsafe_adjustment(increment);
            // Correctly interpreting 0 as success.
            if res == 0 {
                Ok(())
            } else {
                Err("Adjustment failed")
            }
        }
    }
}

// A helper function that uses the Controller, intended to be invoked from tests.
fn simulate(start: i32, increment: i32) -> Result<i32, &'static str> {
    let mut c = Controller { total: start };
    c.adjust(increment)?;
    Ok(c.total)
}

fn main() {
    // Use a valid initial value that allows the unsafe operation to succeed.
    let controller = Arc::new(Mutex::new(Controller { total: 1 }));
    let controller_clone = Arc::clone(&controller);
    let handle = thread::spawn(move || {
        let mut c = controller_clone.lock().unwrap();
        c.adjust(10).unwrap();
    });
    handle.join().unwrap();
    println!("Total: {}", controller.lock().unwrap().total);
}