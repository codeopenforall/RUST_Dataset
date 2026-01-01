//////////////////////////////
// CWE-617 Sample Application - Secured Version
//////////////////////////////
use std::env;

struct Engine {
    counter: usize,
}

impl Engine {
    fn new() -> Self {
        Engine { counter: 0 }
    }

    // The update method now validates the increment safely before applying it.
    // This prevents attacker input from bypassing the safety invariant.
    fn update(&mut self, inc: usize) -> usize {
        // Validate the operation beforehand.
        if self.counter.checked_add(inc).map_or(true, |tmp| tmp >= 100) {
            // Instead of triggering an assertion on attacker influence,
            // we handle the error gracefully by simply not performing the update.
            // In a real-world scenario, proper error signaling would be used.
            println!("Update rejected: operation would exceed threshold.");
            return self.counter;
        }
        // Use safe addition.
        self.counter += inc;
        self.counter
    }
}

fn run_app(input: String) -> usize {
    // Parse input robustly; in production, additional error handling might be warranted.
    let value: usize = input.parse().unwrap();
    let mut eng = Engine::new();
    eng.update(value)
}

fn main() {
    // Accept a command-line argument.
    let arg = env::args().nth(1).unwrap_or("0".to_string());
    let result = run_app(arg);
    println!("Final counter value: {}", result);
}