use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    fn new(n: usize) -> Self {
        let mut data = Vec::with_capacity(n);
        for i in 0..n {
            data.push(i as u32);
        }
        Buffer { data }
    }

    fn safe_read(&self, idx: usize) -> Option<u32> {
        // Validate index prior to performing any unsafe operation.
        if idx < self.data.len() {
            unsafe { Some(*self.data.get_unchecked(idx)) }
        } else {
            None
        }
    }
    
    // Public accessor used by the test oracle.
    // It uses proper bounds checking to avoid out-of-bound memory accesses.
    pub fn read_value_public(&self, idx: usize) -> Option<u32> {
        if idx < self.data.len() {
            unsafe { Some(*self.data.get_unchecked(idx)) }
        } else {
            None
        }
    }
}

fn main() {
    let buffer = Arc::new(Buffer::new(10));
    let mut handles = vec![];

    // Spawn threads to perform reads concurrently.
    for i in 0..5 {
        let buf_clone = Arc::clone(&buffer);
        handles.push(thread::spawn(move || {
            // Even if an out-of-bound index is specified,
            // safe_read returns None and unwrap_or provides a default value.
            let index = if i == 2 { 12 } else { i };
            buf_clone.safe_read(index).unwrap_or(0)
        }));
    }

    for handle in handles {
        let res = handle.join().unwrap();
        println!("Output: {}", res);
    }
}