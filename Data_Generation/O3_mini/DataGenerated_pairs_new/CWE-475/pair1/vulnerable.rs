///////////////////////////////////////////////
// Vulnerable Code Sample for CWE-475
// This module contains an unsafe implementation that
// may trigger undefined behavior when the input count exceeds
// the actual number of elements in the internal vector.
///////////////////////////////////////////////

use std::ptr;

struct DataBuffer {
    buf: Vec<i32>,
}

impl DataBuffer {
    fn new(data: Vec<i32>) -> Self {
        DataBuffer { buf: data }
    }

    // Computes the sum of elements by reading count elements.
    // Precondition: count must not exceed buf.len()
    // Vulnerability: No bounds check is enforced before performing
    // pointer arithmetic, leading to potential undefined behavior.
    fn compute_sum(&self, count: usize) -> i32 {
        let mut sum = 0;
        unsafe {
            let ptr = self.buf.as_ptr();
            for i in 0..count {
                // No verification: if i >= self.buf.len(), this is UB.
                sum += *ptr.add(i);
            }
        }
        sum
    }
}

// Exposed API for integration and testing.
pub fn run() -> i32 {
    let db = DataBuffer::new(vec![1, 2, 3, 4]);
    // Intentionally passing count larger than actual vec length,
    // triggering undefined behavior.
    db.compute_sum(5)
}

fn main() {
    let result = run();
    println!("Result: {}", result);
}