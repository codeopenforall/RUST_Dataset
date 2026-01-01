///////////////////////////////////////////////
// Corrected Code Sample for CWE-475
// This module fixes the unsafe pointer arithmetic by enforcing a precondition
// check to ensure that the input count does not exceed the actual number of elements.
///////////////////////////////////////////////

use std::ptr;

struct DataBuffer {
    buf: Vec<i32>,
}

impl DataBuffer {
    fn new(data: Vec<i32>) -> Self {
        DataBuffer { buf: data }
    }

    // Computes the sum of elements after validating that the count is within bounds.
    // If the count exceeds the stored vector length, the function panics.
    fn compute_sum(&self, count: usize) -> i32 {
        // Precondition check to avoid out-of-bound memory accessing.
        if count > self.buf.len() {
            panic!("Input count {} exceeds buffer length {}", count, self.buf.len());
        }
        let mut sum = 0;
        unsafe {
            let ptr = self.buf.as_ptr();
            for i in 0..count {
                sum += *ptr.add(i);
            }
        }
        sum
    }
}

// Exposed API for integration and testing.
pub fn run() -> i32 {
    let db = DataBuffer::new(vec![1, 2, 3, 4]);
    // Now using count equal to the actual vector length to satisfy precondition.
    db.compute_sum(4)
}

fn main() {
    let result = run();
    println!("Result: {}", result);
}