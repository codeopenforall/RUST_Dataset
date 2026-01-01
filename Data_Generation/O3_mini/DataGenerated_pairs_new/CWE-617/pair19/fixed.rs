/* Fixed Rust code for CWE-617: Reachable Assertion.
   This corrected version properly validates the attacker-supplied index before performing unsafe pointer arithmetic.
   It also avoids triggering a panic by handling out-of-range or unacceptable values gracefully.
   The same concurrent structure and internal design are maintained to mimic real-world usage. */
use std::env;
use std::sync::Arc;
use std::thread;

struct Processor {
    inner: Arc<Vec<u8>>,
}

impl Processor {
    fn new() -> Self {
        // Initialized with the same buffer.
        Processor {
            inner: Arc::new(vec![1, 2, 3, 4, 5]),
        }
    }

    fn execute(&self, idx: usize) {
        // First, check that the index is within bounds.
        if idx >= self.inner.len() {
            eprintln!("Input error: index {} out of range", idx);
            return;
        }
        // Safe branch: perform the unsafe read only after verifying the index.
        unsafe {
            let ptr = self.inner.as_ptr();
            let value = *ptr.add(idx);
            // Instead of asserting, we check the value and handle unacceptable values gracefully.
            if value >= 3 {
                eprintln!("Rejected input: value {} is too high", value);
                return;
            }
            println!("Processed value: {}", value);
        }
    }
}

fn main() {
    // Attacker-controlled input via command-line argument.
    let args: Vec<String> = env::args().collect();
    let idx: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    let proc_obj = Processor::new();

    // Spawn a thread to simulate concurrent processing.
    let shared = proc_obj.inner.clone();
    let handler = thread::spawn(move || {
        let local_proc = Processor { inner: shared };
        local_proc.execute(idx);
    });
    handler.join().unwrap();

    println!("Main completed in the corrected version");
}