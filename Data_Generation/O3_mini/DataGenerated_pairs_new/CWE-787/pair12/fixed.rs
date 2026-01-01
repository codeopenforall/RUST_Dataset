////////////////////////////////////////////////////////////////////////
// Corrected Code Example
////////////////////////////////////////////////////////////////////////
use std::ptr;

struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    // Creates a vector pre-filled with zeros so that its length equals its capacity.
    // This prevents misuse of set_len.
    fn new(size: usize) -> Self {
        Buffer { data: vec![0u8; size] }
    }

    // Safely writes input bytes into the internal buffer, checking that the write
    // does not exceed the buffer’s bounds. Panics if the write would be out-of-bound.
    pub unsafe fn write_data(&mut self, offset: usize, input: &[u8]) {
        if offset + input.len() > self.data.len() {
            panic!("Out-of-bound write prevented");
        }
        // It is now safe to get the raw pointer because the slice is within bounds.
        let dst = self.data.as_mut_ptr().add(offset);
        ptr::copy_nonoverlapping(input.as_ptr(), dst, input.len());
    }
}

fn main() {
    // Create a buffer with size 15.
    let mut buf = Buffer::new(15);
    let input_data = b"HelloWorld";
    // The write is now bounded by the allocated vector (length=15).
    // If an out-of-bound attempt is made, the function will panic, preventing corruption.
    unsafe {
        buf.write_data(5, input_data);
    }
    println!("Buffer content: {:?}", buf.data);
}