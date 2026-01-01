use std::vec::Vec;

struct DataBuffer {
    data: Vec<u8>,
}

impl DataBuffer {
    // Correctly creates a new buffer with capacity n.
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        unsafe {
            // FIX: Set the length exactly to n.
            v.set_len(n);
        }
        DataBuffer { data: v }
    }

    // Fills the buffer completely with the provided value.
    fn fill(&mut self, value: u8) {
        // Iterates over the full length of the buffer.
        for i in 0..self.data.len() {
            self.data[i] = value;
        }
    }
}

fn main() {
    // The input here is chosen consistently.
    let mut buf = DataBuffer::new(5);
    buf.fill(10);
    println!("Buffer contents: {:?}", buf.data);
}