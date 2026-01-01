use std::sync::{Arc, Mutex};
use std::thread;

struct Manager {
    amount: i32,
}

impl Manager {
    // This unsafe function returns 0 on success and a nonzero error code when the operation cannot be applied.
    unsafe fn unsafe_modification(&mut self, increment: i32) -> i32 {
        if self.amount == 0 {
            1 // error code if the current state is invalid
        } else {
            self.amount += increment;
            0 // success code
        }
    }

    // Here the return value from the unsafe call is misinterpreted.
    fn modify(&mut self, increment: i32) -> Result<(), &'static str> {
        unsafe {
            let res = self.unsafe_modification(increment);
            // Vulnerability (Incorrect Check of Function Return Value): 
            // The code treats any nonzero value (i.e. an error) as success.
            if res != 0 {
                Ok(())  // Erroneously indicating success when res is nonzero.
            } else {
                Err("Modification failed")
            }
        }
    }
}

// A helper function that uses the Manager, intended to be invoked from tests.
fn simulate(start: i32, increment: i32) -> Result<i32, &'static str> {
    let mut m = Manager { amount: start };
    m.modify(increment)?;
    Ok(m.amount)
}

fn main() {
    // Using an initial value that, in the unsafe call, triggers an error condition.
    // In the vulnerable code, even though unsafe_modification returns an error code (nonzero),
    // the check in modify mistakenly reports success.
    let manager = Arc::new(Mutex::new(Manager { amount: 0 }));
    let manager_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        let mut m = manager_clone.lock().unwrap();
        // In this scenario, modify is incorrectly considered successful even though amount is 0.
        m.modify(10).unwrap();
    });
    handle.join().unwrap();
    println!("Amount: {}", manager.lock().unwrap().amount);
}