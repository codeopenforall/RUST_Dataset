//////////////////////////////////////////////
// Corrected Implementation
//////////////////////////////////////////////
use std::sync::Arc;

struct Processor {
    data: Arc<Vec<i32>>,
}

trait Calculation {
    fn compute(&self, start: usize, count: usize) -> Result<i32, &'static str>;
}

impl Calculation for Processor {
    fn compute(&self, start: usize, count: usize) -> Result<i32, &'static str> {
        let slice = &self.data;
        // Ensure that the range [start, start+count) lies within the vector.
        if start.checked_add(count).map_or(true, |end| end > slice.len()) {
            return Err("Index out-of-bounds");
        }
        let mut total = 0;
        // Safe access using slice.get() that returns an Option.
        for i in 0..count {
            total += slice.get(start + i).unwrap();
        }
        Ok(total)
    }
}

fn main() {
    // Create a data buffer with exactly four integers.
    let proc_inst = Processor { data: Arc::new(vec![10, 20, 30, 40]) };
    // Triggering input: starting at index 3, request 2 elements.
    match proc_inst.compute(3, 2) {
        Ok(result) => println!("Computed sum: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}