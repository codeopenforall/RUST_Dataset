use std::sync::{Arc, Mutex};
use std::thread;

struct BufferWrapper {
    buffer: Vec<u8>,
}

impl BufferWrapper {
    fn new(size: usize) -> Self {
        // Allocate with a given capacity and unsafely set the length.
        let mut v = Vec::with_capacity(size);
        unsafe { v.set_len(size); }
        BufferWrapper { buffer: v }
    }

    fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        // Vulnerability: no check is made that 'index' is within bounds.
        // This unsafe pointer arithmetic can write past the allocated memory.
        unsafe {
            let ptr = self.buffer.as_mut_ptr().add(index);
            *ptr = value;
        }
        Ok(())
    }
}

fn main() {
    let wrapper = Arc::new(Mutex::new(BufferWrapper::new(10)));
    let wrapper_clone = Arc::clone(&wrapper);
    let handle = thread::spawn(move || {
        let mut lock = wrapper_clone.lock().unwrap();
        // The call below writes to index 15, which is outside the allocated buffer (0..10),
        // leading to an out-of-bounds memory write.
        let _ = lock.update(15, 55);
    });
    handle.join().unwrap();
    println!("Execution complete: Vulnerability present.");
}