//////////////////////////////
// Vulnerable Code Snippet
//////////////////////////////
use std::slice;

struct Processor {
    data: Vec<usize>,
}

impl Processor {
    fn new(data: Vec<usize>) -> Self {
        Processor { data }
    }

    // Calculates the total by unsafely constructing a slice using the provided length.
    // This violates the API precondition if the user-supplied length exceeds the
    // actual number of elements in the vector, resulting in reading beyond the allocated memory.
    fn compute_total(&self, len: usize) -> usize {
        unsafe {
            // Get the raw pointer to the vector's memory.
            let ptr = self.data.as_ptr();
            // POTENTIAL FLAW:
            // Directly use the user-supplied length to form a slice without validating it.
            let part = slice::from_raw_parts(ptr, len);
            part.iter().sum()
        }
    }
}

fn main() {
    // Setup with 5 known elements.
    let proc_inst = Processor::new(vec![10, 20, 30, 40, 50]);
    // Simulate external input that violates the precondition:
    // The input length (7) is greater than the actual number of elements (5).
    let result = proc_inst.compute_total(7);
    println!("Total computed: {}", result);
}