//
// Vulnerable code for CWE-670: Always-Incorrect Control Flow Implementation
// This implementation uses unsafe blocks and concurrency with shared mutable state.
// The program is intended to update an internal flag to "authorized" when the input
// exceeds a threshold. However, the control flow in the update function is inverted:
// it sets the flag to "unauthorized" in the expected "allow" branch and vice versa.
//
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

    // The update method contains an unsafe block with flawed control flow:
    // it intentionally reverses the expected branches.
    fn update(&self, input: i32) {
        unsafe {
            // Flawed control flow: when input > 100, it should mark as Allowed,
            // but it erroneously selects Blocked, and vice versa.
            let new_state = match input > 100 {
                true => Access::Blocked,   // BUG: should be Allowed.
                false => Access::Allowed,  // BUG: should be Blocked.
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

    // Spawn threads to update shared flag concurrently.
    for _ in 0..5 {
        let clone = Arc::clone(&shared_handler);
        let t = thread::spawn(move || {
            // Using a triggering input (150) which should grant access.
            clone.update(150);
        });
        handles.push(t);
    }

    for t in handles {
        t.join().unwrap();
    }

    // Based on correct logic, access should be granted for input 150.
    // However, this vulnerable version prints "Access denied" due to flawed control flow.
    if shared_handler.check() {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}