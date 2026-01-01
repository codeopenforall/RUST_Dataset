///////////////////////////////////////////////////////////////////////////////
// This Rust program corrects the off‐by‐one bug by ensuring that only the valid
// n elements are written to the buffer and summed. The loop now correctly 
// iterates from 0 to n (exclusive), preventing any writes past allocated memory.
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
            // Correct loop: iterating only over valid indices.
            for i in 0..n {
                *ptr.add(i) = input[i];
            }
            // Properly set the length to n.
            storage.set_len(n);
        }
        // Sum the values safely using the vector’s iterator.
        storage.iter().sum()
    }
}

fn main() {
    let proc = Processor;
    let data = vec![1, 2, 3, 4];
    let result = proc.execute(&data);
    println!("Result: {}", result);
}