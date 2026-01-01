//////////////////////
// Vulnerable Code
//////////////////////
use std::sync::Arc;
use std::thread;

struct Processor {
    buffer: Vec<u32>,
}

impl Processor {
    fn new() -> Self {
        Processor { buffer: vec![100, 200, 300, 400, 500] }
    }

    fn operate(&self, idx: usize) -> Option<u32> {
        // Vulnerable unsafe access: no bounds checking is performed.
        unsafe {
            // CWE-125: Out-of-Bounds Read. If idx is not within 0..buffer.len(), undefined behavior occurs.
            Some(*self.buffer.as_ptr().add(idx))
        }
    }
}

fn run_op() -> Option<u32> {
    let proc = Processor::new();
    // Here, idx 5 is intentionally chosen. Since valid indices are 0..5, this access is out-of-bound.
    proc.operate(5)
}

fn main() {
    let shared = Arc::new(Processor::new());
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        run_op()
    });
    match handle.join() {
        Ok(result) => println!("Operation result: {:?}", result),
        Err(_) => println!("Thread panicked"),
    }
}