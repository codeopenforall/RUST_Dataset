///////////////////////
// Fixed Example
///////////////////////
struct DataHolder {
    data: Vec<u32>,
}

impl DataHolder {
    // Creates a new holder with the given capacity.
    fn new(capacity: usize) -> Self {
        Self {
            // Directly allocate the vector with capacity; no unsound initialization.
            data: Vec::with_capacity(capacity),
        }
    }

    // Safely fills the buffer with the provided value.
    // Ensures that writes do not exceed the allocated capacity.
    pub fn inject_safe(&mut self, count: usize, value: u32) {
        if count > self.data.capacity() {
            panic!("Count {} exceeds capacity {}", count, self.data.capacity());
        }
        // Clear any previous contents.
        self.data.clear();
        // Use safe push operations to ensure proper bounds-checking.
        for _ in 0..count {
            self.data.push(value);
        }
    }

    // Computes the sum of the data elements.
    pub fn compute(&self) -> u32 {
        self.data.iter().sum()
    }
}

// This function demonstrates the fixed behavior and returns the computed sum.
fn run() -> u32 {
    let mut holder = DataHolder::new(10);
    holder.inject_safe(10, 42);
    holder.compute()
}

fn main() {
    let total = run();
    println!("Sum: {}", total);
}