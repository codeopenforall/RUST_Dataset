//////////////////////////////
// Fixed Code Example
//////////////////////////////
use std::thread;
use std::sync::Arc;

struct Engine;

impl Engine {
    // The compute function now uses checked multiplication to detect overflow.
    // If overflow occurs, it returns None.
    fn compute(&self, a: i32, b: i32) -> Option<i32> {
        a.checked_mul(b)  // [Line 16]
    }
}

fn run_calculation() {
    let engine = Arc::new(Engine);
    let handles: Vec<_> = (0..4).map(|_| {
        let calc = Arc::clone(&engine);
        thread::spawn(move || {
            // Multiplying two numbers that would overflow in i32
            calc.compute(50000, 50000)
        })
    }).collect();

    for handle in handles {
        let result = handle.join().unwrap();
        // In the fixed version, overflow is detected and None is returned.
        println!("Computed value: {:?}", result);
    }
}

fn main() {
    println!("Starting fixed computation...");
    run_calculation();
}