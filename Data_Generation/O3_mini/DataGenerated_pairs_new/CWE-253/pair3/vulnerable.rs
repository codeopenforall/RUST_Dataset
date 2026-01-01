/////////////////////// Vulnerable Code ///////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

// This unsafe function simulates an external call.
// It returns 0 for a successful operation and 1 to indicate an error.
unsafe fn external_process(value: i32) -> i32 {
    if value > 100 {
        1
    } else {
        0
    }
}

struct Engine {
    flag: Arc<Mutex<bool>>,
}

impl Engine {
    fn new() -> Self {
        Engine {
            flag: Arc::new(Mutex::new(false)),
        }
    }

    // The method spawns a thread and calls an unsafe function.
    // Vulnerability: It misinterprets the return value of external_process.
    // A non-zero return (indicating error) is treated as success.
    fn process(&self, value: i32) -> bool {
        let flag = self.flag.clone();
        let handle = thread::spawn(move || {
            let res = unsafe { external_process(value) };
            // INCORRECT check: a non-zero result indicates an error,
            // but here it is treated as a successful outcome.
            if res != 0 {
                let mut f = flag.lock().unwrap();
                *f = true;
            } else {
                let mut f = flag.lock().unwrap();
                *f = false;
            }
        });
        handle.join().unwrap();
        *self.flag.lock().unwrap()
    }
}

fn main() {
    let engine = Engine::new();
    // Triggering input: For value 150, external_process returns 1 (error),
    // but the incorrect check treats it as success.
    let outcome = engine.process(150);
    if outcome {
        println!("Operation succeeded");
    } else {
        println!("Operation failed");
    }
}