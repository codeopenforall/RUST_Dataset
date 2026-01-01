use std::ptr;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    fn new(len: usize) -> Self {
        // Allocate exact memory for the given number of elements.
        Self { data: vec![0; len] }
    }

    fn compute(&mut self) -> u32 {
        let len = self.data.len();
        unsafe {
            let ptr = self.data.as_mut_ptr();
            for i in 0..len {
                // Off-by-one error: writing to i+1 instead of i.
                // When i == len - 1, this writes one element past the allocated area.
                ptr.add(i + 1).write(1);
            }
        }
        self.data.iter().sum()
    }
}

fn main() {
    // Using a buffer length that will trigger an out-of-bound write.
    let mut buf = Buffer::new(6);
    let result = buf.compute();
    println!("{}", result);
}