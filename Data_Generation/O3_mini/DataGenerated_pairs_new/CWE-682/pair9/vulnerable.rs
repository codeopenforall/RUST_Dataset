////////////////////// Vulnerable Code //////////////////////
use std::thread;
use std::sync::Arc;

struct Processor {
    data: Vec<u32>,
    scale: u32,
}

impl Processor {
    // This method computes a scaled average using an incorrect arithmetic order.
    // It intentionally calculates the result as (sum / count) * scale instead of (sum * scale) / count,
    // which can yield a wrong value when the division truncates before multiplication.
    fn compute(&self) -> u32 {
        let sum: u32 = self.data.iter().sum();
        let count = self.data.len() as u32;
        // Unsafe block used to read the computed sum pointer (simulating a scenario from real-world unsafe APIs)
        let ptr: *const u32 = &sum;
        let unsafe_sum = unsafe { *ptr };
        // Vulnerability: Incorrect operator precedence leads to truncation error.
        let result = unsafe_sum / count * self.scale;
        result
    }
}

fn main() {
    // Using a small dataset to trigger the arithmetic error vulnerability.
    let proc_obj = Arc::new(Processor { data: vec![1, 2], scale: 100 });
    let mut threads = vec![];
    for _ in 0..2 {
        let proc_clone = Arc::clone(&proc_obj);
        let handle = thread::spawn(move || {
            let output = proc_clone.compute();
            println!("Computed result: {}", output);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}