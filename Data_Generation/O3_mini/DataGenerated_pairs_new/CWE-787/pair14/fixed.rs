use std::ptr;

pub struct Handler {
    buf: Vec<u8>,
}

impl Handler {
    pub fn new(size: usize) -> Self {
        // Allocate and initialize the buffer safely.
        let buf = vec![0u8; size];
        Self { buf }
    }

    // This update method performs explicit boundary checking.
    // If the provided index is out-of-bounds, the function returns an error.
    pub fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        if index < self.buf.len() {
            self.buf[index] = value;
            Ok(())
        } else {
            Err("index out of bounds")
        }
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        self.buf.get(index).copied()
    }
}

fn main() {
    let mut handler = Handler::new(10);
    // Attempt to update an index outside the valid bounds.
    // The fixed code checks the bounds and returns an error.
    match handler.update(10, 255) {
        Ok(_) => println!("Main (fixed): update succeeded unexpectedly."),
        Err(e) => println!("Main (fixed): error occurred: {}", e),
    }
}