//////////////////////////////
// Vulnerable Code Start
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
    // This method performs an unchecked division in an unsafe block.
    // If the divisor is zero, this will invoke undefined arithmetic behavior (panic at runtime).
    fn compute(&self, divisor: i32) -> i32 {
        unsafe {
            // Vulnerability: Division by zero is not checked.
            let result = self.value / divisor;
            result
        }
    }
}

fn main() {
    let engine = Arc::new(Engine::new(100));
    let engine_clone = engine.clone();

    // Spawn a thread to simulate concurrent use.
    let handle = thread::spawn(move || {
        // The triggering input here is zero, which leads to a division by zero.
        let bad_input = 0;
        let res = engine_clone.compute(bad_input);
        println!("Computed result: {}", res);
    });

    let _ = handle.join();
    println!("Execution completed");
}
//////////////////////////////
// Vulnerable Code End
//////////////////////////////