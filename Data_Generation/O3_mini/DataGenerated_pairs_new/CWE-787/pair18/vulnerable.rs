/* 
   This Rust program contains an unsafe out‐of‐bounds write vulnerability.
   It creates a buffer with a capacity of N but mistakenly sets its length to N+1,
   and then writes to the last (unallocated) element without boundary checks.
   This mimics a real-world CWE-787 scenario.
*/
use std::thread;

struct BufferHandler {
    buffer: Vec<u32>,
}

impl BufferHandler {
    // Creates a buffer of size `size` but erroneously sets length to size+1.
    fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        // UNSAFE: Incorrectly extend the length beyond capacity.
        unsafe { buffer.set_len(size + 1); }
        // Initialize only the first 'size' elements.
        for i in 0..size {
            buffer[i] = 0;
        }
        Self { buffer }
    }

    // Performs an unsafe update without bounds checking.
    fn update(&mut self, index: usize, value: u32) {
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            // Vulnerability: This can write out-of-bounds when index equals size.
            *ptr.add(index) = value;
        }
    }

    // Returns the sum of the buffer.
    fn aggregate(&self) -> u32 {
        self.buffer.iter().sum()
    }
}

fn process() -> u32 {
    let mut handler = BufferHandler::new(10);
    // Spawn a thread to perform the update concurrently.
    let join_handle = thread::spawn(move || {
        // Intentionally update the element at index 10 (i.e. one past the proper bounds)
        handler.update(10, 42);
        handler.aggregate()
    });
    join_handle.join().unwrap()
}

fn main() {
    let outcome = process();
    println!("Result: {}", outcome);
}