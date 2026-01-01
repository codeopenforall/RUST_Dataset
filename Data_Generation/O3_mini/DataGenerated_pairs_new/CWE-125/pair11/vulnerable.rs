//////////////////////////////////////////////
// Vulnerable Implementation
//////////////////////////////////////////////
use std::sync::Arc;

struct Processor {
    data: Arc<Vec<i32>>,
}

trait Calculation {
    fn compute(&self, start: usize, count: usize) -> i32;
}

impl Calculation for Processor {
    fn compute(&self, start: usize, count: usize) -> i32 {
        // Insecure: no bounds checking. Using unsafe get_unchecked to mimic an out‐of‐bounds read.
        let slice = &self.data;
        let mut total = 0;
        unsafe {
            for i in 0..count {
                // POTENTIAL FLAW: If start+i is >= slice.len(), this reads out-of-bounds.
                total += *slice.get_unchecked(start + i);
            }
        }
        total
    }
}

fn main() {
    // Create a data buffer with exactly four integers.
    let proc_inst = Processor { data: Arc::new(vec![10, 20, 30, 40]) };
    // Triggering input: starting at index 3, request 2 elements.
    // For a 4-element vector, index 3 is valid but index 4 is out-of-bounds.
    let result = proc_inst.compute(3, 2);
    println!("Computed sum: {}", result);
}