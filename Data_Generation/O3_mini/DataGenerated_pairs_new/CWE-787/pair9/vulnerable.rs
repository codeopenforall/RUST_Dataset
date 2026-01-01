//////////////////////////////
// Vulnerable Code Example
//////////////////////////////
use std::thread;

struct DataHolder {
    buffer: Vec<u32>,
}

impl DataHolder {
    // Creates a DataHolder with a fixed capacity of 10.
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(10),
        }
    }

    // Processes the input slice by writing its items into the buffer using an unsafe block.
    // If the input length exceeds the allocated capacity, this leads to out‐of‐bounds writes.
    pub fn process(&mut self, input: &[u32]) {
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            // Iterate over each element in the input and write to the allocated memory.
            // There is no bounds check: if input.len() > capacity, memory beyond the allocation is written.
            for (i, &val) in input.iter().enumerate() {
                *ptr.add(i) = val;
            }
            // Set the internal length without verifying that it is within capacity.
            self.buffer.set_len(input.len());
        }
    }

    pub fn total(&self) -> u32 {
        self.buffer.iter().fold(0, |sum, &x| sum + x)
    }
}

fn main() {
    let mut holder = DataHolder::new();
    // Triggering input: 11 elements (one more than capacity) will lead to an out‐of‐bounds write.
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    holder.process(&input);
    println!("Main thread total: {}", holder.total());

    // Also demonstrate concurrent usage.
    let input_shared = input.clone();
    let handle = thread::spawn(move || {
        let mut worker = DataHolder::new();
        worker.process(&input_shared);
        println!("Thread total: {}", worker.total());
    });
    handle.join().unwrap();
}