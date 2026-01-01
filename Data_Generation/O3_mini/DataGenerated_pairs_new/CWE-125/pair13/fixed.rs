use std::sync::Arc;
use std::thread;

struct Processor {
    data: Box<[u32]>,
}

impl Processor {
    fn compute(&self, start: usize, count: usize) -> u32 {
        // Explicitly verify that the requested range is within the allocated bounds.
        if start.checked_add(count).unwrap_or(usize::MAX) > self.data.len() {
            panic!("Index out-of-bounds in compute");
        }
        // Safe slicing leverages Rust's built-in bounds checks.
        let slice = &self.data[start..start + count];
        slice.iter().sum()
    }
}

fn main() {
    let proc = Processor { data: Box::new([10, 20, 30, 40, 50]) };
    let arc_proc = Arc::new(proc);
    let handles: Vec<_> = (0..3).map(|_| {
        let p = Arc::clone(&arc_proc);
        thread::spawn(move || {
            // Now using indices that exceed bounds triggers an explicit panic.
            p.compute(2, 4)
        })
    }).collect();

    for handle in handles {
        // Threads that panic will be handled gracefully.
        let _ = handle.join().unwrap_or_else(|err| {
            eprintln!("Thread panicked: {:?}", err);
            0
        });
    }
    println!("Completed execution.");
}