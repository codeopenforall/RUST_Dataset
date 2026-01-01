/*
   This revised Rust program addresses the integer overflow issue (CWE-190) by
   performing checked multiplication. In the fixed version, if input * 1000 would overflow,
   the function returns an error rather than producing an incorrect result. It also preserves
   the realistic usage pattern with concurrency.
*/
use std::thread;

struct Processor;

impl Processor {
    // The fixed compute method performs checked arithmetic.
    pub fn compute(input: u32) -> Result<u64, &'static str> {
        // Perform checked multiplication to prevent overflow.
        let capacity: u32 = input.checked_mul(1000).ok_or("overflow detected")?;
        
        // Use a thread to simulate concurrency in processing.
        let handle = thread::spawn(move || {
            let cap_u64 = capacity as u64;
            // Use checked arithmetic where possible.
            // Since capacity is verified to be correct, this calculation should not overflow.
            cap_u64
                .checked_mul(cap_u64.checked_sub(1).unwrap())
                .and_then(|v| v.checked_div(2))
                .unwrap_or(0)
        });
        match handle.join() {
            Ok(result) => Ok(result),
            Err(_) => Err("Thread panicked"),
        }
    }
}

fn main() {
    // The same input is used.
    let input: u32 = 5_000_000;
    match Processor::compute(input) {
        Ok(result) => {
            println!("Computed result: {}", result);
        }
        Err(e) => {
            // The expected behavior in the fixed version is to catch the overflow and return an error.
            println!("Error: {}", e);
        }
    }
}