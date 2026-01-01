////////////////////////////////////////////////////////////////////////
// Vulnerable Code Example
////////////////////////////////////////////////////////////////////////
use std::ptr;

struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    // Creates a vector with allocated capacity but zero length.
    // This allows later unsafe extension via set_len.
    fn new(capacity: usize) -> Self {
        Buffer { data: Vec::with_capacity(capacity) }
    }

    // UNSAFE operation: writes input bytes into the internal buffer
    // without proper bounds-checking. If offset + input.len() exceeds
    // the original allocation, set_len is misused and the vector’s
    // length is updated unsafely, leading to an out-of-bound write.
    pub unsafe fn write_data(&mut self, offset: usize, input: &[u8]) {
        // Vulnerability starts here.
        let dst = self.data.as_mut_ptr().add(offset);                    // Line 14
        ptr::copy_nonoverlapping(input.as_ptr(), dst, input.len());        // Line 15
        self.data.set_len(offset + input.len());                           // Line 16
        // Vulnerability ends.
    }
}

fn main() {
    // Create a buffer with capacity 10 (but length is 0)
    let mut buf = Buffer::new(10);
    // "HelloWorld" is 10 bytes; writing it at offset 5 will extend the length to 15,
    // which is beyond the allocated capacity.
    let input_data = b"HelloWorld";
    unsafe {
        buf.write_data(5, input_data);
    }
    // Reading the vector now may access uninitialized or corrupt memory.
    println!("Buffer content: {:?}", buf.data);
}