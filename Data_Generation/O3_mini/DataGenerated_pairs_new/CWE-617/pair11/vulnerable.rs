//////////////////////////////
// CWE-617 Sample Application
//////////////////////////////
use std::env;

struct Engine {
    counter: usize,
}

impl Engine {
    fn new() -> Self {
        Engine { counter: 0 }
    }

    // This routine takes an increment from external input and uses an unsafe block to update
    // the counter. Afterwards it asserts that the counter is still below a hardcoded threshold.
    // Because the increment comes from an attacker‐controlled input, it is possible to trigger
    // the assertion and cause a denial of service.
    fn update(&mut self, inc: usize) -> usize {
        // Unsafe pointer manipulation
        unsafe {
            let ptr: *mut usize = &mut self.counter;
            // Bypass Rust’s strict aliasing rules by directly writing with wrapping_add.
            // Note: No proper check is made to ensure the result is below the safe threshold.
            *ptr = self.counter.wrapping_add(inc);
        }
        // Reachable assertion using attacker-influenced state.
        // If inc is high enough to bring counter to 100 or more, the assertion is triggered.
        assert!(self.counter < 100, "Counter exceeded safe threshold");
        self.counter
    }
}

// Exposed function used by main (do not include in test oracle)
fn run_app(input: String) -> usize {
    // Parse input that may be attacker controlled.
    let value: usize = input.parse().unwrap();
    let mut eng = Engine::new();
    eng.update(value)
}

fn main() {
    // Accept a command-line argument (attack vector) and run the application.
    let arg = env::args().nth(1).unwrap_or("0".to_string());
    let result = run_app(arg);
    println!("Final counter value: {}", result);
}