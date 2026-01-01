/* 
This corrected Rust program fixes the control flow in the controller's update routine.
Now, when the flag is true, the counter is correctly incremented, and when false, it is decremented.
The flag is updated properly based on the resulting counter value.
The program still uses unsafe blocks and spawns threads as in the vulnerable version.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct Controller {
    flag: bool,
    counter: i32,
}

impl Controller {
    pub fn new() -> Self {
        // Initially, the flag is set to true and the counter to 0.
        Controller { flag: true, counter: 0 }
    }

    // Corrected update routine: branch logic properly reflects intended behavior.
    pub fn update(&mut self) {
        unsafe {
            let flag_ptr: *mut bool = &mut self.flag;
            // Correct behavior: increment if flag is true, decrement if false.
            if *flag_ptr {
                self.counter = self.counter.wrapping_add(1);
            } else {
                self.counter = self.counter.wrapping_sub(1);
            }
            // Properly update the flag: if the counter is negative, set flag to false; otherwise true.
            if self.counter < 0 {
                *flag_ptr = false;
            } else {
                *flag_ptr = true;
            }
        }
    }
}

fn main() {
    let controller = Arc::new(Mutex::new(Controller::new()));
    let mut handles = vec![];

    for _ in 0..10 {
        let ctrl = Arc::clone(&controller);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut c = ctrl.lock().unwrap();
                c.update();
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    let c = controller.lock().unwrap();
    println!("Final counter: {}, flag: {}", c.counter, c.flag);
}