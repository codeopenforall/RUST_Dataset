use std::vec::Vec;

struct DataBuffer {
    data: Vec<u8>,
}

impl DataBuffer {
    // Creates a new buffer with capacity n.
    fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        unsafe {
            // FLAW: Off-by-one error via setting the length to n+1.
            // This exposes an uninitialized element and may lead to memory unsafety.
            v.set_len(n + 1);
        }
        DataBuffer { data: v }
    }

    // Fills the buffer (except the last element, assuming it to be a guard)
    // with the provided value.
    fn fill(&mut self, value: u8) {
        // Intentionally iterates until len()-1, expecting exactly n initialized elements,
        // but due to off-by-one, one element remains uninitialized.
        for i in 0..self.data.len() - 1 {
            self.data[i] = value;
        }
    }
}

fn main() {
    // The input here is chosen to trigger the fencepost error.
    let mut buf = DataBuffer::new(5);
    buf.fill(10);
    // The printed vector contains one extra element that is uninitialized.
    println!("Buffer contents: {:?}", buf.data);
}