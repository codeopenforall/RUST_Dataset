///////////////////////
// Vulnerable Example
///////////////////////
use std::fmt;

struct DataHolder {
    data: Vec<u32>,
}

impl DataHolder {
    // Creates a new holder with the given capacity.
    // Initializes the underlying vector to a known size.
    fn new(capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        // Pre-fill with zeros to allow pointer arithmetic; note that this unsafely
        // sets the length without proper initialization.
        unsafe {
            vec.set_len(capacity);
        }
        Self { data: vec }
    }

    // Unsafe function that fills the buffer with a specified value.
    // CWE-787: Out-of-Bounds Write vulnerability.
    // The loop intentionally iterates from 0 to count inclusive,
    // writing count+1 elements into a buffer that is only set to length count.
    pub unsafe fn inject(&mut self, count: usize, value: u32) {
        let ptr = self.data.as_mut_ptr();
        // Artificially set the vector's length to 'count' so that later
        // accesses beyond count may become out-of-bounds.
        self.data.set_len(count);
        // Intentional off-by-one: loop goes from 0 to count inclusive.
        for i in 0..=count {
            // Vulnerability: when i == count, this writes beyond the allocated bounds.
            ptr.add(i).write(value);
        }
    }

    // Computes the sum of the data elements.
    pub fn compute(&self) -> u32 {
        self.data.iter().sum()
    }
}

// This function drives the demonstration and returns the computed sum.
fn run() -> u32 {
    let mut holder = DataHolder::new(10);
    unsafe {
        // Trigger the vulnerability: passing count = 10 writes 11 elements.
        holder.inject(10, 42);
    }
    holder.compute()
}

fn main() {
    let total = run();
    println!("Sum: {}", total);
}