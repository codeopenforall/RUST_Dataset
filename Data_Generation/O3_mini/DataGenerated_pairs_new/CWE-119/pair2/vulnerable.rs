//////////////////////////////////////////////
// Vulnerable version demonstrating an off‐by‐one memory bounds issue
// using unsafe pointer arithmetic and concurrent unsynchronized access.
// This implementation allocates a vector of (n+1) bytes, where the (n+1)th
// byte is intended as a sentinel. However, an off‐by‐one error in the
// unsafe fill method mistakenly writes into the sentinel region.
use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Vec<u8>,
    size: usize, // logical number of elements to fill
}

unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}

impl Buffer {
    // Creates a new buffer with a capacity of (size + 1)
    // so that the extra slot can serve as a sentinel.
    fn new(size: usize) -> Self {
        let data = vec![0u8; size + 1];
        Buffer { data, size }
    }

    // Unsafe fill writes a given value into indices 0..=size.
    // The loop boundary is off by one, as it writes into the sentinel slot.
    unsafe fn fill(&self, value: u8) {
        let ptr = self.data.as_ptr() as *mut u8;
        // Vulnerability: Iterates inclusive of self.size, corrupting the sentinel.
        for i in 0..=self.size {
            *ptr.add(i) = value;
        }
    }

    // Provides read access for testing purposes.
    fn get(&self, index: usize) -> u8 {
        self.data[index]
    }
}

fn main() {
    let buffer = Arc::new(Buffer::new(10));
    let buffer_clone = Arc::clone(&buffer);
    // Spawn a thread to concurrently invoke the unsafe fill.
    let handle = thread::spawn(move || {
        unsafe {
            buffer_clone.fill(0xAA);
        }
    });
    unsafe {
        buffer.fill(0xAA);
    }
    handle.join().unwrap();
    println!("Finished operation");
}