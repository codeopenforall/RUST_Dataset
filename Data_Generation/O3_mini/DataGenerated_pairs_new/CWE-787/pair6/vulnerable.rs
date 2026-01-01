//////////////////////////////////////////////////////////////
// This Rust program contains a vulnerability corresponding to
// CWE-787: Out-of-Bounds Write. It defines a buffer type that is
// internally represented by a Vec<u32> which is unsafely extended 
// using set_len. Its update method “assign” uses unchecked pointer 
// arithmetic. Consequently, when an index equal to or larger than the 
// allocated size is provided, the code performs an out‐of‐bounds write.
//////////////////////////////////////////////////////////////
use std::ptr;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        // Allocate capacity and unsafely define the vector length without proper initialization.
        let mut v = Vec::with_capacity(size);
        unsafe {
            // Vulnerability: Incorrectly sets the length to the capacity without careful validation.
            v.set_len(size);
        }
        Buffer { data: v }
    }

    fn assign(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        unsafe {
            // Vulnerability: The pointer arithmetic does not perform a bounds check.
            let ptr = self.data.as_mut_ptr();
            // This can write past the allocated memory if index >= self.data.len()
            *ptr.add(index) = value;
        }
        Ok(())
    }

    fn content(&self) -> &[u32] {
        &self.data
    }
}

fn main() {
    let mut buf = Buffer::new(10);
    // This call attempts to write at index 10 (valid indices: 0..9)
    // This triggers an out-of-bounds write vulnerability.
    match buf.assign(10, 999) {
        Ok(()) => println!("Assignment completed (vulnerable)."),
        Err(e) => println!("Error: {}", e),
    }
    println!("Buffer content: {:?}", buf.content());
}