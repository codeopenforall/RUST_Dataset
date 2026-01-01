/* 
This Rust program simulates a controller that updates an internal counter based on a flag.
It uses unsafe blocks for direct pointer manipulation and spawns multiple threads.
Due to an always-incorrect control flow, the update routine uses inverted branch logic:
when the flag is true it erroneously decrements the counter instead of incrementing it,
and then improperly resets the flag based on the counter value.
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

    // Flawed update routine: the branch logic is inverted.
    pub fn update(&mut self) {
        unsafe {
            let flag_ptr: *mut bool = &mut self.flag;
            // Vulnerability: Instead of incrementing when flag is true, it decrements.
            if *flag_ptr {
                self.counter = self.counter.wrapping_sub(1);
            } else {
                self.counter = self.counter.wrapping_add(1);
            }
            // Then, the flag is set inversely according to the counter value.
            if self.counter < 0 {
                *flag_ptr = true;
            } else {
                *flag_ptr = false;
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