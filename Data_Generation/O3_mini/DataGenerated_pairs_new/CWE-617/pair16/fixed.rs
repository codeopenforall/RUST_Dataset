//////////////////////////////////////////////////////////////
// Corrected Code: CWE-617 - Ensuring Input Validation and Safe Concurrency
//////////////////////////////////////////////////////////////
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct System {
    threshold: usize,
}

impl System {
    fn new(value: usize) -> Self {
        System { threshold: value }
    }

    // The invariant is now enforced in the input validation.
    // In debug builds, a debug_assert checks the invariant.
    fn process(&self) {
        debug_assert!(self.threshold < 100, "Threshold too high, possible DoS");
    }
}

// The configuration is now stored safely using shared ownership and mutual exclusion.
fn run(args: Vec<String>) {
    let input = args.get(1).expect("Missing argument");
    let value: usize = input.parse().expect("Invalid number provided");

    // Validate input to ensure that the invariant holds before proceeding.
    if value >= 100 {
        panic!("Invalid threshold value");
    }

    let config = Arc::new(Mutex::new(System::new(value)));

    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let sys = config_clone.lock().unwrap();
        sys.process();
    });

    {
        let sys = config.lock().unwrap();
        sys.process();
    }

    handle.join().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    run(args);
}