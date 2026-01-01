/* 
   This Rust program suffers from an integer wraparound vulnerability (CWE-190).
   A multiplication is performed inside an unsafe block using wrapping arithmetic,
   so for large inputs the multiplication silently overflows rather than triggering an error.
   The computed “capacity” is then used to derive a sum, causing the program to return an incorrect result.
*/
use std::thread;

struct Processor;

impl Processor {
    // The function performs a multiplication without checking for overflow.
    // It is intended to compute: capacity = input * 1000 and then sum = capacity*(capacity-1)/2.
    pub fn compute(input: u32) -> Result<u64, &'static str> {
        // Using an unsafe block to simulate low-level unchecked arithmetic (e.g. similar to pointer arithmetic).
        let capacity: u32 = unsafe {
            // Vulnerability: wrapping_mul silently wraps on integer overflow.
            input.wrapping_mul(1000)
        };
        // Parallel simulation: spawn a thread to calculate the sum.
        let handle = thread::spawn(move || {
            // Compute sum in a way that uses wrapping_sub and unchecked division.
            // If overflow occurred above then capacity is much lower than intended.
            let cap_u64 = capacity as u64;
            let sum = (cap_u64 * cap_u64.wrapping_sub(1)) / 2;
            sum
        });
        // Join the thread and return the computed sum.
        match handle.join() {
            Ok(result) => Ok(result),
            Err(_) => Err("Thread panicked"),
        }
    }
}

fn main() {
    // This input is chosen such that input * 1000 exceeds u32::MAX,
    // so the multiplication overflows in the vulnerable version.
    let input: u32 = 5_000_000;
    match Processor::compute(input) {
        Ok(result) => {
            // The result is computed based on the overflowed capacity.
            println!("Computed result: {}", result);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}