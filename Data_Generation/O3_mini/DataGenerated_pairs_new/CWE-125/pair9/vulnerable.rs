#![allow(unused)]
use std::sync::Arc;
use std::thread;

struct Processor {
    data: Vec<u32>,
}

impl Processor {
    fn new(data: Vec<u32>) -> Self {
        Processor { data }
    }

    // This method unsafely reads an element from the internal vector.
    // It uses pointer arithmetic without any bounds checking.
    fn process_at(&self, index: usize) -> Result<u32, &'static str> {
        // Vulnerability: No check is performed, so if "index" is out of bounds,
        // the unsafe pointer dereference leads to an out-of-bound read (CWE-125).
        unsafe {
            Ok(*self.data.as_ptr().add(index))
        }
    }
}

// This function spawns a thread that concurrently calls the processing routine.
fn execute(index: usize) -> Result<u32, &'static str> {
    let proc = Processor::new(vec![10, 20, 30, 40]);
    let proc_arc = Arc::new(proc);
    let proc_clone = Arc::clone(&proc_arc);
    let handle = thread::spawn(move || {
        proc_clone.process_at(index)
    });
    handle.join().unwrap()
}

fn main() {
    // Pass an index equal to the length of the vector (i.e. 4),
    // which is out-of-bound since valid indices are 0..3.
    // In this vulnerable version, the unsafe read is performed without checking.
    let idx = 4;
    // The result is wrapped in Ok even if the underlying memory read is undefined.
    match execute(idx) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}