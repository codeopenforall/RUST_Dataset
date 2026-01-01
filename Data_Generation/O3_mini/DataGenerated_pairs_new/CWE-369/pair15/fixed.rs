//////////////////////////////
// Corrected Implementation
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    value: i32,
}

impl Engine {
    fn new(val: i32) -> Self {
        Self { value: val }
    }

    // The corrected version of exec first checks if the divisor is zero.
    // If so, it returns an error rather than performing an unsafe division.
    fn exec(&self, divisor: i32) -> Result<i32, &'static str> {
        if divisor == 0 {
            return Err("division by zero");
        }
        unsafe {
            let result = self.value / divisor;
            Ok(result)
        }
    }
}

fn main() {
    // Use Arc and Mutex to simulate concurrent access.
    let engine = Arc::new(Mutex::new(Engine::new(100)));
    let engine_clone = Arc::clone(&engine);
    let handle = thread::spawn(move || {
        let engine = engine_clone.lock().unwrap();
        // Instead of panicking on zero, the function returns an error.
        let res = engine.exec(0);
        assert!(res.is_err(), "Should return error on division by zero");
    });
    let _ = handle.join();
    println!("Operation completed safely.");
}