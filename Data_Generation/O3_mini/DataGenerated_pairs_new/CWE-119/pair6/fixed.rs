//////////////////////
// Fixed Code
//////////////////////
use std::thread;

struct MemoryHandler {
    buffer: *mut u32,
    capacity: usize,
}

impl MemoryHandler {
    pub fn new(size: usize) -> Self {
        // Allocate a boxed slice with specified size.
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, 0);
        let boxed = vec.into_boxed_slice();
        let ptr = Box::into_raw(boxed) as *mut u32;
        MemoryHandler {
            buffer: ptr,
            capacity: size,
        }
    }

    // Checked write that enforces the index is within bounds.
    pub fn write_checked(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index < self.capacity {
            unsafe {
                *self.buffer.add(index) = value;
            }
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub unsafe fn read(&self, index: usize) -> u32 {
        *self.buffer.add(index)
    }

    // Free the allocated memory.
    pub fn free(self) {
        unsafe {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(self.buffer, self.capacity));
        }
    }
}

// This function performs the operation with index verification.
// If the index is not valid, it returns an error.
pub fn process_operation(index: usize) -> Result<u32, &'static str> {
    let mut handler = MemoryHandler::new(10);
    handler.write_checked(index, 99)?;
    let res = unsafe { handler.read(index) };
    handler.free();
    Ok(res)
}

fn main() {
    // Spawn a thread to simulate concurrent access.
    let handle = thread::spawn(|| {
        // In-bound operation.
        let r = process_operation(5);
        println!("Thread operation result: {:?}", r);
    });
    // This main thread call uses an invalid out-of-bound index.
    let result = process_operation(10);
    handle.join().unwrap();
    println!("Main thread operation result: {:?}", result);
}