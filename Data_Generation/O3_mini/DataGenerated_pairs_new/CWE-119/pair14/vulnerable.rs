///////////////////////////////////////////////////////////
// Vulnerable Code: Buffer manipulation with unsafe bounds //
///////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Buffer {
    // A boxed slice with a fixed size.
    data: Box<[u32]>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        let vec = vec![0u32; size].into_boxed_slice();
        Buffer { data: vec }
    }

    // Unsafely writes a value at the given index without checking bounds.
    fn write_data(&mut self, idx: usize, value: u32) {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            // Vulnerability: No check on 'idx' leads to potential out-of-bound write.
            *ptr.add(idx) = value;
        }
    }

    // Unsafely reads a value at the given index without validating the index.
    fn read_data(&self, idx: usize) -> u32 {
        unsafe {
            let ptr = self.data.as_ptr();
            // Vulnerability: No bounds check.
            *ptr.add(idx)
        }
    }
}

trait Process {
    // Processes the buffer by summing 'count' elements unsafely.
    fn process(&self, count: usize) -> u32;
}

impl Process for Buffer {
    fn process(&self, count: usize) -> u32 {
        let mut sum: u32 = 0;
        unsafe {
            let ptr = self.data.as_ptr();
            // Vulnerability: Iterates 'count' times, which may exceed allocation.
            for i in 0..count {
                sum = sum.wrapping_add(*ptr.add(i));
            }
        }
        sum
    }
}

// This function encapsulates the computation.
// It spawns a thread that unsafely writes to the buffer.
// The unsafe write may corrupt memory if 'input' is out-of-bound.
fn execute(input: usize) -> u32 {
    let size = 10;
    let buffer = Arc::new(Mutex::new(Buffer::new(size)));
    {
        let buffer_clone = Arc::clone(&buffer);
        thread::spawn(move || {
            let mut buf = buffer_clone.lock().unwrap();
            // Vulnerable: If 'input' >= size this write is out-of-bound.
            buf.write_data(input, 42);
        })
        .join()
        .unwrap();
    }
    let buf = buffer.lock().unwrap();
    // Process 'input + 1' elements, possibly triggering an out-of-bound read.
    buf.process(input + 1)
}

fn main() {
    // Using boundary value: with size=10 valid indices are 0..9.
    // Passing 10 causes an out-of-bound write/read.
    let input = 10;
    let result = execute(input);
    println!("Result: {}", result);
}