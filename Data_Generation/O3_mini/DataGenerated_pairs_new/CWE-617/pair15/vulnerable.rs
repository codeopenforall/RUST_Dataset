//////////////////////////// Vulnerable Code ////////////////////////////
// This code defines a common interface for processing an input index
// over a private collection. It unsafely reads an element using raw pointer
// arithmetic and then uses an assert!() to validate that the value meets a
// minimum threshold. If an attacker supplies an index (e.g., 0) that selects a
// low value (in this case 5), the reachable assertion is triggered, causing a
// panic and a potential denial‐of‐service.
use std::env;
use std::ptr;

trait Computable {
    fn compute(&self, input: usize) -> Result<u32, &'static str>;
}

struct Handler {
    data: Vec<u32>,
}

impl Handler {
    fn new() -> Self {
        // Note: The first element is intentionally below the safe threshold.
        Self { data: vec![5, 15, 25] }
    }
}

impl Computable for Handler {
    fn compute(&self, idx: usize) -> Result<u32, &'static str> {
        // Unsafe block: Reads memory from the internal vector without checking bounds.
        let ptr = self.data.as_ptr();
        let value = unsafe { *ptr.add(idx) };
        // Reachable assertion: if the value is below 10 (attacker influence), the assertion triggers.
        assert!(value >= 10, "Value too low: vulnerability triggered.");
        Ok(value)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Accept attacker-supplied index; default is 0.
    let idx = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        0
    };
    let handler = Handler::new();
    // A panic here (due to the assert!) constitutes the exploitable vulnerability.
    let result = handler.compute(idx).unwrap();
    println!("Computed value: {}", result);
}
///////////////////////// End Vulnerable Code /////////////////////////