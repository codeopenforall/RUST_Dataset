//////////////////////////////
// Vulnerable Rust Code
//////////////////////////////
use std::sync::Arc;
use std::thread;

pub struct Processor {
    // Pointer to a u32 buffer segment; count denotes how many elements to read.
    ptr: *const u32,
    count: usize,
}

impl Processor {
    // Constructs a Processor from a slice.
    // ERROR: It deliberately calculates the pointer by subtracting one,
    // violating the precondition of pointer arithmetic: the pointer becomes invalid.
    pub fn build(buffer: &[u32]) -> Self {
        // Here, subtracting 1 from the slice's base pointer is undefined behavior.
        let ptr = unsafe { buffer.as_ptr().sub(1) };
        // Fixed count: reading 3 elements.
        Processor { ptr, count: 3 }
    }

    // Returns the sum of the "count" elements starting from the raw pointer.
    pub fn compute(&self) -> u32 {
        // Create a slice from the raw pointer without checking that it's valid.
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
    // Build the processor with an invalid pointer (subtracting one).
    let proc_inst = Processor::build(&data);
    let data_clone = data.clone();
    // Spawn a thread simulating concurrent usage.
    let handle = thread::spawn(move || {
        // The thread uses the processor to compute a total.
        proc_inst.compute()
    });
    let result = handle.join().expect("Thread panicked");
    println!("Computed total: {}", result);
    result
}

fn main() {
    // Calling calculate_total will likely lead to undefined behavior.
    let _ = calculate_total();
}