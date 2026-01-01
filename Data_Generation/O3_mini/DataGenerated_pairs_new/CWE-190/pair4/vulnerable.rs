//////////////////////////////
// Vulnerable Code Example
//////////////////////////////
#![allow(dead_code)]
use std::thread;
use std::sync::Arc;

struct Engine;

impl Engine {
    // The compute function uses an unsafe unchecked multiplication intrinsic.
    // It does not detect integer overflow.
    fn compute(&self, a: i32, b: i32) -> Option<i32> {
        // Vulnerability: Using unchecked multiplication can produce a wrapped result.
        unsafe {
            let res = std::intrinsics::unchecked_mul(a, b);  // [Line 16]
            Some(res)
        }
    }
}

fn run_calculation() {
    let engine = Arc::new(Engine);
    let handles: Vec<_> = (0..4).map(|_| {
        let calc = Arc::clone(&engine);
        thread::spawn(move || {
            // Multiplying two numbers that cause overflow.
            // Expected mathematical result: 50000 * 50000 = 2,500,000,000 (which doesn't fit in i32).
            calc.compute(50000, 50000)
        })
    }).collect();

    for handle in handles {
        let result = handle.join().unwrap();
        // Print the computed result (wrapped-around value)
        println!("Computed value: {:?}", result);
    }
}

fn main() {
    println!("Starting vulnerable computation...");
    run_calculation();
}