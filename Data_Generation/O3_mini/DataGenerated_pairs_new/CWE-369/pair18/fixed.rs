//////////////////////////////
// Corrected Code Start
//////////////////////////////
use std::sync::Arc;
use std::thread;

struct Engine {
    value: i32,
}

impl Engine {
    fn new(val: i32) -> Self {
        Self { value: val }
    }
    // This method now checks the divisor against zero before performing division.
    // If a zero divisor is provided, it safely returns the original value to avoid a panic.
    fn compute(&self, divisor: i32) -> i32 {
        if divisor == 0 {
            // Instead of performing division, return a safe default value.
            // In real applications, it might be preferable to return a Result or log an error.
            self.value
        } else {
            self.value / divisor
        }
    }
}

fn main() {
    let engine = Arc::new(Engine::new(100));
    let engine_clone = engine.clone();

    // Spawn a thread to simulate concurrent usage.
    let handle = thread::spawn(move || {
        let input = 0;
        let res = engine_clone.compute(input);
        println!("Computed result: {}", res);
    });

    let _ = handle.join();
    println!("Execution completed");
}
//////////////////////////////
// Corrected Code End
//////////////////////////////