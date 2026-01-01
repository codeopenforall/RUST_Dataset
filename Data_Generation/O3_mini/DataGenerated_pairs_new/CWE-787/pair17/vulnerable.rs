//////////////////////////////
// Vulnerable version code
//////////////////////////////

use std::ptr;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    // Creates a new buffer with the specified number of elements.
    fn new(size: usize) -> Self {
        Buffer { data: vec![0; size] }
    }
    
    // Fills every element in the buffer with the given value.
    // 
    // Vulnerability: This function performs an out‐of‐bounds write.
    // The unsafe block iterates from 0 to len (inclusive) and writes to
    // an index equal to len, and then sets the vector length to len+1.
    // This leads to memory corruption and undefined behavior.
    fn fill_data(&mut self, value: u32) {
        unsafe {
            let n = self.data.len();
            let ptr = self.data.as_mut_ptr();
            // Out-of-bounds write: iterates through 0..=n, writing to index n.
            for i in 0..=n {
                *ptr.add(i) = value;
            }
            // Incorrectly extends the vector's length, marking an uninitialized element as valid.
            self.data.set_len(n + 1);
        }
    }
}

fn main() {
    let mut buf = Buffer::new(10);
    buf.fill_data(55);
    println!("Buffer content: {:?}", buf.data);
}