///////////////////////////////////////////////////////////////////////////////
// This Rust program contains an off‐by‐one bug in an unsafe block that writes
// one extra element to a buffer and then mistakenly uses a raw slice to sum
// one element too many. This extra element (a hard‐coded value 999) is used 
// inadvertently in the computation, resulting in an incorrect sum.
///////////////////////////////////////////////////////////////////////////////
use std::vec::Vec;

struct Processor;

impl Processor {
    pub fn execute(&self, input: &[u32]) -> u32 {
        let n = input.len();
        // Allocate a vector with capacity for exactly n elements.
        let mut storage: Vec<u32> = Vec::with_capacity(n);
        unsafe {
            let ptr = storage.as_mut_ptr();
            // Off-by-one loop: writing n+1 values.
            for i in 0..=n {  // Vulnerability: iterating from 0 to n (inclusive)
                if i < n {
                    // Correct copy for valid indices.
                    *ptr.add(i) = input[i];
                } else {
                    // Erroneously write a magic value at one-past-the-end.
                    *ptr.add(i) = 999;
                }
            }
            // Only set length to n, hiding the extra write.
            storage.set_len(n);
            // BUG: Incorrectly create a slice of n+1 elements, reading the extra value.
            let computed = std::slice::from_raw_parts(storage.as_ptr(), n + 1)
                            .iter()
                            .sum::<u32>();
            computed
        }
    }
}

fn main() {
    let proc = Processor;
    let data = vec![1, 2, 3, 4];
    let result = proc.execute(&data);
    println!("Result: {}", result);
}