//////////////////////////////////////////////
// Fixed Version: Sanitized Input to Prevent DoS via Reachable Assertion
// This version validates the attacker-supplied input before using it to initialize shared state.
// The input is sanitized by clamping values exceeding the threshold, ensuring that the assertion in operate()
// is never triggered.
//////////////////////////////////////////////
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

const THRESHOLD: u32 = 1000;

struct Data {
    value: u32,
}

impl Data {
    // Unsafe initialization remains for low-level mimicry.
    unsafe fn init(val: u32) -> Self {
        Data { value: val }
    }

    // The same operation is used, but with the guarantee that "value" is safe.
    fn operate(&self) {
        // With proper input sanitization this assertion should never fail.
        assert!(self.value < THRESHOLD, "Reachable assertion triggered: value too high");
    }
}

pub fn execute_app(input: u32) {
    // Input sanitization: clamp the value to below the THRESHOLD.
    let safe_input = if input < THRESHOLD { input } else { THRESHOLD - 1 };
    let shared = Arc::new(Mutex::new(unsafe { Data::init(safe_input) }));
    let cloned = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let data = cloned.lock().unwrap();
        data.operate();
    });
    handle.join().unwrap();
    let data_main = shared.lock().unwrap();
    data_main.operate();
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