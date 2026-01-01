//////////////////////////////
// Vulnerable Code Example
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Machine {
    active: bool,
    counter: u32,
}

impl Machine {
    // Unsafe initialization method that indicates whether the machine is ready.
    // It should be checked by the caller.
    unsafe fn init(&mut self) -> Result<(), &'static str> {
        if self.active {
            Ok(())
        } else {
            Err("machine inactive")
        }
    }
    
    // Operation method that runs a sequence of actions.
    // Vulnerability: The result of the unsafe init call is ignored,
    // which leads to continued processing even if the machine is inactive.
    fn operate(&self, shared: Arc<Mutex<Machine>>) -> i32 {
        {
            // Vulnerable usage: ignoring the Result returned by init.
            let mut mach = shared.lock().unwrap();
            let _ = unsafe { mach.init() };
            mach.counter += 1; // Continues processing regardless of initialization failure.
        }
        // Spawn a thread that performs an additional update.
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let mut mach = shared_clone.lock().unwrap();
            mach.counter += 2;
        });
        handle.join().unwrap();
        let mach = shared.lock().unwrap();
        mach.counter as i32
    }
}

pub fn execute() -> Result<i32, &'static str> {
    // Create a Machine that is inactive.
    let machine = Machine { active: false, counter: 0 };
    let shared = Arc::new(Mutex::new(machine));
    // Call operate: the error from init is ignored, leading to an incorrect operation.
    let result = {
        let mach = shared.lock().unwrap();
        mach.operate(Arc::clone(&shared))
    };
    // Even though initialization should have failed, a value is computed.
    Ok(result)
}

fn main() {
    match execute() {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}