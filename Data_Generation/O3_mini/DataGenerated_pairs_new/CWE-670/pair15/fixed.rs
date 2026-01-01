//
// Fixed code for CWE-670: Always-Incorrect Control Flow Implementation
// This corrected version fixes the branch inversion by properly mapping inputs > 100 to allow access,
// and inputs <= 100 to block access. Unsafe usage remains for demonstration, but the control flow now
// correctly updates the shared flag.
//
// Imports.
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Access {
    Allowed,
    Blocked,
}

struct Handler {
    flag: Arc<Mutex<Access>>,
}

impl Handler {
    fn new() -> Self {
        Handler {
            flag: Arc::new(Mutex::new(Access::Blocked)),
        }
    }

    // Fixed update method: properly selects the state based on the input.
    fn update(&self, input: i32) {
        unsafe {
            // Correct control flow:
            // If input > 100, set to Allowed; otherwise, set to Blocked.
            let new_state = match input > 100 {
                true => Access::Allowed,
                false => Access::Blocked,
            };
            let mut current = self.flag.lock().unwrap();
            *current = new_state;
        }
    }

    fn check(&self) -> bool {
        let current = self.flag.lock().unwrap();
        match *current {
            Access::Allowed => true,
            Access::Blocked => false,
        }
    }
}

fn main() {
    let handler = Handler::new();
    let shared_handler = Arc::new(handler);
    let mut handles = vec![];

    // Spawn threads to concurrently update the shared flag.
    for _ in 0..5 {
        let clone = Arc::clone(&shared_handler);
        let t = thread::spawn(move || {
            // Using the triggering input (150) that should grant access.
            clone.update(150);
        });
        handles.push(t);
    }

    for t in handles {
        t.join().unwrap();
    }

    // With the fixed control flow, access is correctly granted for input >100.
    if shared_handler.check() {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}