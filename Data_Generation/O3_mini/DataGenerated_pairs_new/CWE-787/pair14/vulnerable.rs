use std::ptr;

pub struct Handler {
    buf: Vec<u8>,
}

impl Handler {
    pub fn new(size: usize) -> Self {
        // Allocate with capacity and manually set length (contents uninitialized)
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        Self { buf }
    }

    // This function writes a value to an index without checking bounds.
    // It is vulnerable to out-of-bounds writes (CWE-787).
    pub fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        unsafe {
            let ptr = self.buf.as_mut_ptr();
            // Vulnerability: no bounds validation, allowing writes outside the vector.
            // Even if index == buf.len(), the write occurs on unowned memory.
            ptr.add(index).write(value);
        }
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        self.buf.get(index).copied()
    }
}

fn main() {
    let mut handler = Handler::new(10);
    // Triggering the vulnerability: writing at index 10 which is out-of-bounds.
    // In well-defined code, valid indices are 0..10, but here no check is performed.
    let _ = handler.update(10, 255);
    // Unsafe read to display the value; may read corrupted memory or cause UB.
    unsafe {
        let val = *handler.buf.as_ptr().add(10);
        println!("Main (vulnerable): value at index 10 is {}", val);
    }
}