//////////////////////////////
// Fixed Code Snippet
//////////////////////////////
use std::cmp;

struct Processor {
    data: Vec<usize>,
}

impl Processor {
    fn new(data: Vec<usize>) -> Self {
        Processor { data }
    }

    // Safely calculates the total by first validating the length against the actual data size.
    // If the supplied length exceeds the size of the vector, it only sums the available elements.
    fn compute_total(&self, len: usize) -> usize {
        // Enforce the API precondition: do not allow overshooting the allocated data.
        let valid_len = cmp::min(len, self.data.len());
        // Use safe slicing, ensuring bounds are not exceeded.
        self.data[..valid_len].iter().sum()
    }
}

fn main() {
    // Setup with 5 known elements.
    let proc_inst = Processor::new(vec![10, 20, 30, 40, 50]);
    // Even when given an input length of 7, the function now corrects it to the available element count.
    let result = proc_inst.compute_total(7);
    println!("Total computed: {}", result);
}