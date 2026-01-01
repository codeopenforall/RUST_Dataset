//////////////////////////////
// Fixed version code
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
    // Fix: The unsafe block now only iterates over valid indices [0, len)
    // and does not manually change the vector length.
    fn fill_data(&mut self, value: u32) {
        unsafe {
            let n = self.data.len();
            let ptr = self.data.as_mut_ptr();
            // Safe iteration: only indexes 0..n (excludes out-of-bounds index).
            for i in 0..n {
                *ptr.add(i) = value;
            }
            // No modification to vector length; the buffer remains valid.
        }
    }
}

fn main() {
    let mut buf = Buffer::new(10);
    buf.fill_data(55);
    println!("Buffer content: {:?}", buf.data);
}