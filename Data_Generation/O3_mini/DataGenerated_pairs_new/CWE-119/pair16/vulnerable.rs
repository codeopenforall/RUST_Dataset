/////////////////////////////////////////////////////////////
// This code demonstrates a memory‐safety vulnerability.
// A custom buffer structure is defined and manipulated
// using an unsafe function that writes to a computed pointer.
// The unsafe function does not verify that the index is within
// the valid bounds of the vector.  This may lead to out‐of‐bounds
// memory writes (CWE-119), similar in nature to real-world issues
// reported in RustSec.
/////////////////////////////////////////////////////////////

use std::ptr;

struct MemoryBuffer {
    data: Vec<u32>,
}

impl MemoryBuffer {
    // Creates a new buffer of the given size initialized with zeros.
    fn new(size: usize) -> Self {
        Self { data: vec![0; size] }
    }
    
    // UNSAFE: Directly writes a value at the given index using raw pointer arithmetic.
    // No bounds checking is performed; thus an index equal to or greater than data.len()
    // results in a memory safety bug.
    pub unsafe fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        let ptr = self.data.as_mut_ptr();
        *ptr.add(idx) = val; // Vulnerability: no bounds validation here
        Ok(())
    }
}

// Entry-point function that performs the update with an out-of-bound index.
fn run() -> Result<(), &'static str> {
    let mut buf = MemoryBuffer::new(10);
    // The following call uses an index which is exactly equal to the buffer length,
    // leading to an out-of-bound memory write.
    unsafe { buf.update(10, 100) }?;
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => println!("Operation succeeded"),
        Err(e) => println!("Error: {}", e),
    }
}