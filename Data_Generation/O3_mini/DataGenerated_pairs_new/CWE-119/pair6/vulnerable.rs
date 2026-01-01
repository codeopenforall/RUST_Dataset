//////////////////////
// Vulnerable Code
//////////////////////
use std::thread;

struct MemoryHandler {
    buffer: *mut u32,
    capacity: usize,
}

impl MemoryHandler {
    pub fn new(size: usize) -> Self {
        // Allocate a boxed slice and leak its pointer for raw manipulation.
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, 0);
        let boxed = vec.into_boxed_slice();
        let ptr = Box::into_raw(boxed) as *mut u32;
        MemoryHandler {
            buffer: ptr,
            capacity: size,
        }
    }

    // Unsafe write without bounds checking.
    pub unsafe fn write_unchecked(&self, index: usize, value: u32) {
        // Vulnerability: writing without verifying the index; this may write out of bounds.
        *self.buffer.add(index) = value;
    }

    pub unsafe fn read(&self, index: usize) -> u32 {
        *self.buffer.add(index)
    }

    // Free the allocated memory.
    pub fn free(self) {
        unsafe {
            // Reconstruct the boxed slice to drop the memory.
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(self.buffer, self.capacity));
        }
    }
}

// This function performs an operation that writes a value at the given index.
// In the vulnerable version, no index validation is performed.
pub fn process_operation(index: usize) -> Result<u32, &'static str> {
    let handler = MemoryHandler::new(10);
    unsafe {
        // The following unsafe write does not check bounds.
        handler.write_unchecked(index, 99);
        let res = handler.read(index);
        handler.free();
        Ok(res)
    }
}

fn main() {
    // Spawn a thread to simulate concurrent access.
    let handle = thread::spawn(|| {
        // In-bound index operation.
        let r = process_operation(5);
        println!("Thread operation result: {:?}", r);
    });
    // This main thread call uses an out-of-bound index (10 is invalid for capacity 10).
    let result = process_operation(10);
    handle.join().unwrap();
    println!("Main thread operation result: {:?}", result);
}