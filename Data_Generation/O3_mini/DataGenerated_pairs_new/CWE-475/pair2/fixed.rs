//////////////////////////////
// Corrected Rust Code
//////////////////////////////
use std::sync::Arc;
use std::thread;

pub struct Processor {
    // Pointer to a u32 buffer segment; count denotes how many elements to read.
    ptr: *const u32,
    count: usize,
}

impl Processor {
    // Constructs a Processor from a slice in a safe manner.
    // It verifies that the intended pointer arithmetic remains within bounds.
    pub fn build(buffer: &[u32]) -> Option<Self> {
        // Instead of subtracting 1, we use the valid pointer.
        // Ensure that we have at least `count` elements available.
        let count = 3;
        if buffer.len() < count {
            return None;
        }
        let ptr = buffer.as_ptr();
        Some(Processor { ptr, count })
    }

    // Returns the sum of the "count" elements starting from the raw pointer.
    pub fn compute(&self) -> u32 {
        // Create a slice from the validated pointer.
        unsafe {
            std::slice::from_raw_parts(self.ptr, self.count)
                .iter()
                .fold(0, |acc, &num| acc.wrapping_add(num))
        }
    }
}

pub fn calculate_total() -> u32 {
    // Create a sample buffer.
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    // Build the processor safely; if parameters are invalid, exit.
    let proc_inst = Processor::build(&data).expect("Buffer does not meet size requirements");
    let data_clone = data.clone();
    // Spawn a thread simulating concurrent usage.
    let handle = thread::spawn(move || {
        proc_inst.compute()
    });
    let result = handle.join().expect("Thread panicked");
    println!("Computed total: {}", result);
    result
}

fn main() {
    // Calling calculate_total safely computes the total.
    let _ = calculate_total();
}