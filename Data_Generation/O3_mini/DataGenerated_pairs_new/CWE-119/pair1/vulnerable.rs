use std::ptr;

struct Container {
    inner: Vec<u8>,
}

impl Container {
    fn new(size: usize) -> Self {
        Container { inner: vec![0; size] }
    }

    // This function performs unchecked memory writes.
    fn modify(&mut self, start: isize, count: usize, val: u8) -> Result<(), &'static str> {
        // Vulnerability: No validation of the memory bounds before performing raw pointer writes.
        unsafe {
            let ptr = self.inner.as_mut_ptr().offset(start);
            for i in 0..count {
                // Out-of-bound writes may occur if start + count exceeds "inner" length.
                *ptr.offset(i as isize) = val;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut obj = Container::new(10);
    // This call intentionally writes 12 bytes into a 10-byte container, triggering memory corruption.
    match obj.modify(0, 12, 42) {
        Ok(_) => println!("Modification completed."),
        Err(e) => println!("Error: {}", e),
    }
}