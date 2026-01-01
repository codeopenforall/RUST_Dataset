use std::sync::Arc;
use std::thread;

struct Processor {
    data: Box<[u32]>,
}

impl Processor {
    fn compute(&self, start: usize, count: usize) -> u32 {
        unsafe {
            // Vulnerability: compute a pointer offset without verifying that the requested range is within bounds.
            let ptr = self.data.as_ptr().add(start); // (Line 12)
            // Vulnerability: constructing a slice from raw parts without validating that (start + count) is within the allocation.
            let slice = std::slice::from_raw_parts(ptr, count); // (Line 14)
            slice.iter().sum()
        }
    }
}

fn main() {
    let proc = Processor { data: Box::new([10, 20, 30, 40, 50]) };
    let arc_proc = Arc::new(proc);
    let handles: Vec<_> = (0..3).map(|_| {
        let p = Arc::clone(&arc_proc);
        thread::spawn(move || {
            // Intentionally request a subrange that extends beyond the buffer end (2 + 4 > 5).
            p.compute(2, 4)
        })
    }).collect();

    for handle in handles {
        // In a real execution this may trigger undefined behavior (reading out‐of‐bound memory).
        let _ = handle.join().unwrap();
    }
    println!("Completed execution.");
}