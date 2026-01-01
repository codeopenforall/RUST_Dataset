use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Box<[i32]>,
}

impl Buffer {
    fn new(v: Vec<i32>) -> Self {
        Buffer { data: v.into_boxed_slice() }
    }

    // Vulnerable method: performs an unchecked read.
    fn access(&self, idx: usize) -> Option<i32> {
        // FLAW: No bounds checking is performed here.
        unsafe {
            // This call uses unchecked pointer arithmetic.
            Some(*self.data.get_unchecked(idx))
        }
    }
}

fn execute(buffer: Arc<Buffer>, idx: usize) -> Option<i32> {
    buffer.access(idx)
}

fn main() {
    let buffer = Arc::new(Buffer::new((0..10).collect()));
    let idx = 15; // Out-of-bound index intended to trigger undefined behavior.
    let handle = {
        let buf = Arc::clone(&buffer);
        thread::spawn(move || {
            execute(buf, idx)
        })
    };

    match handle.join().unwrap() {
        Some(val) => println!("Result: {}", val),
        None => println!("None"),
    }
}