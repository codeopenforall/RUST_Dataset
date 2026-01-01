//////////////////////////////////////////////
// Vulnerable Version: DoS by Reachable Assertion
// This version unsafely accepts user input and uses it without proper sanitization.
// An attacker can supply a value (e.g. 1000 or larger) that triggers a reachable assertion,
// causing the application to panic and denying service.
//////////////////////////////////////////////
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

const THRESHOLD: u32 = 1000;

struct Data {
    value: u32,
}

impl Data {
    // Unsafe initialization mimicking low-level operations.
    unsafe fn init(val: u32) -> Self {
        Data { value: val }
    }

    // Operation that performs a reachable assertion. If the value is not less than THRESHOLD,
    // the assertion will panic, resulting in a denial of service.
    fn operate(&self) {
        // Vulnerability: attacker-controlled "value" can trigger this assertion.
        assert!(self.value < THRESHOLD, "Reachable assertion triggered: value too high");
    }
}

// The application function creates shared state and spawns a thread that
// concurrently calls the operation. Both paths call operate(), which may panic.
pub fn execute_app(input: u32) {
    let shared = Arc::new(Mutex::new(unsafe { Data::init(input) }));
    let cloned = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let data = cloned.lock().unwrap();
        data.operate();  // Vulnerable assertion call in thread
    });
    handle.join().unwrap();
    let data_main = shared.lock().unwrap();
    data_main.operate();  // Vulnerable assertion call in main thread
    println!("Completed");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args
        .get(1)
        .expect("No input provided")
        .parse::<u32>()
        .unwrap();
    execute_app(input);
}