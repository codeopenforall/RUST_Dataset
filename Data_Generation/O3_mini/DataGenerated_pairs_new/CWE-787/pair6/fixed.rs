//////////////////////////////////////////////////////////////
// This corrected Rust program addresses CWE-787 by properly validating 
// index bounds and safely initializing the vector. Instead of using an unsafe 
// set_len, it initializes the vector via resize, and the assign method checks 
// the index before performing the update.
//////////////////////////////////////////////////////////////
use std::ptr;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        // Safely initialize the vector with known values.
        let mut v = Vec::with_capacity(size);
        v.resize(size, 0);
        Buffer { data: v }
    }

    fn assign(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        // Validate that the index is within bounds.
        if index >= self.data.len() {
            return Err("Index out of bounds");
        }
        // Perform safe assignment.
        if let Some(elem) = self.data.get_mut(index) {
            *elem = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    fn content(&self) -> &[u32] {
        &self.data
    }
}

fn main() {
    let mut buf = Buffer::new(10);
    // This call uses a valid index; an attempt to use an invalid index would trigger an error.
    buf.assign(9, 999).expect("Assignment failed");
    println!("Assignment completed (fixed).");
    println!("Buffer content: {:?}", buf.content());
}