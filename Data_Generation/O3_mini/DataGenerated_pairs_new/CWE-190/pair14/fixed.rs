/* This Rust program corrects the arithmetic overflow vulnerability by using checked arithmetic.
   The compute method now verifies that multiplication does not overflow and that the result fits 
   within u16; if not, it returns an error. The design mirrors real-world fixes implemented following 
   RustSec advisories. */
use std::sync::{Arc, Barrier};
use std::thread;

struct Processor {
    factor: u64,
}

impl Processor {
    // This safe method uses checked multiplication and returns a Result.
    fn compute(&self, input: u64) -> Result<u16, &'static str> {
        // Use checked_mul to detect overflow of multiplication.
        let product = input.checked_mul(self.factor).ok_or("Multiplication overflow")?;
        if product > (u16::MAX as u64) {
            return Err("Result exceeds maximum allowed value");
        }
        Ok(product as u16)
    }
}

// Spawns several threads that concurrently compute using the safe routine.
// Each thread returns a Result<u16, &str>, and if any error occurs, run returns the error.
fn run(input: u64) -> Result<u16, &'static str> {
    let proc = Arc::new(Processor { factor: 2 });
    let barrier = Arc::new(Barrier::new(4));
    let mut handles = vec![];
    let mut results = vec![];

    for _ in 0..4 {
        let proc_clone = Arc::clone(&proc);
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            proc_clone.compute(input)
        });
        handles.push(handle);
    }

    for handle in handles {
        let res = handle.join().unwrap()?;
        results.push(res);
    }
    Ok(results.into_iter().sum())
}

fn main() {
    let input = 70000u64;
    match run(input) {
        Ok(total) => println!("Total: {}", total),
        Err(e) => println!("Error: {}", e),
    }
}