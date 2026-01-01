//////////////////////////////
// Fixed Code Example
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Machine {
    active: bool,
    counter: u32,
}

impl Machine {
    // Unsafe initialization method that indicates whether the machine is ready.
    unsafe fn init(&mut self) -> Result<(), &'static str> {
        if self.active {
            Ok(())
        } else {
            Err("machine inactive")
        }
    }
    
    // Operation method that runs a sequence of actions.
    // Fixed: Properly checks the result of the unsafe init call.
    fn operate(&self, shared: Arc<Mutex<Machine>>) -> Result<i32, &'static str> {
        {
            let mut mach = shared.lock().unwrap();
            // Check and propagate the error from init.
            unsafe { mach.init()? };
            mach.counter += 1;
        }
        // Spawn a thread to perform an additional update.
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let mut mach = shared_clone.lock().unwrap();
            mach.counter += 2;
        });
        handle.join().unwrap();
        let mach = shared.lock().unwrap();
        Ok(mach.counter as i32)
    }
}

pub fn execute() -> Result<i32, &'static str> {
    let machine = Machine { active: false, counter: 0 };
    let shared = Arc::new(Mutex::new(machine));
    let result = {
        let mach = shared.lock().unwrap();
        // Propagate errors from operate.
        mach.operate(Arc::clone(&shared))?
    };
    Ok(result)
}

fn main() {
    match execute() {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}