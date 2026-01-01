//////////////////////////////
// Corrected Code Example
//////////////////////////////
use std::thread;

struct DataHolder {
    buffer: Vec<u32>,
}

impl DataHolder {
    // Creates a DataHolder with an initial capacity of 10.
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(10),
        }
    }

    // Processes the input safely. If the input length exceeds the current capacity,
    // then the internal buffer is reallocated with sufficient capacity.
    // Unsafe writing is preserved to mimic low-level operations but is now guarded.
    pub fn process(&mut self, input: &[u32]) {
        // Ensure that the internal buffer has adequate capacity.
        if input.len() > self.buffer.capacity() {
            // Reallocate with the new required capacity.
            self.buffer = Vec::with_capacity(input.len());
        }
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            for (i, &val) in input.iter().enumerate() {
                // Compute index i with guarantee that i is < new capacity.
                *ptr.add(i) = val;
            }
            self.buffer.set_len(input.len());
        }
    }

    pub fn total(&self) -> u32 {
        self.buffer.iter().fold(0, |sum, &x| sum + x)
    }
}

fn main() {
    let mut holder = DataHolder::new();
    // Triggering input: The input length (11 elements) is now handled safely as the buffer is reallocated.
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    holder.process(&input);
    println!("Main thread total: {}", holder.total());

    // Demonstrate correct concurrent usage.
    let input_shared = input.clone();
    let handle = thread::spawn(move || {
        let mut worker = DataHolder::new();
        worker.process(&input_shared);
        println!("Thread total: {}", worker.total());
    });
    handle.join().unwrap();
}