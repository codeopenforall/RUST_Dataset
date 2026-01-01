/* This Rust program demonstrates a flawed arithmetic routine that suffers from integer overflow (CWE-190). 
   The computation multiplies an input by a factor in an unsafe block and then casts the 64‐bit product 
   to a 16‐bit integer without checking for overflow. This mimics real-world unsafe multithreaded code 
   seen in past RustSec advisories. */
use std::sync::{Arc, Barrier};
use std::thread;

struct Processor {
    factor: u64,
}

impl Processor {
    // This unsafe method performs unchecked multiplication and casts the result.
    // If the product exceeds the maximum value of u16, the cast causes wraparound.
    unsafe fn calculate(&self, input: u64) -> u16 {
        // Vulnerability: unchecked multiplication and cast may cause integer overflow.
        let product = input * self.factor;
        product as u16
    }
}

// Spawns several threads that concurrently compute a result with the shared processor.
// The results from each thread are summed and returned.
fn run(input: u64) -> u16 {
    let proc = Arc::new(Processor { factor: 2 });
    let barrier = Arc::new(Barrier::new(4));
    let mut handles = vec![];
    let mut results = vec![];

    for _ in 0..4 {
        let proc_clone = Arc::clone(&proc);
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            unsafe { proc_clone.calculate(input) }
        });
        handles.push(handle);
    }

    for handle in handles {
        results.push(handle.join().unwrap());
    }
    // Sum the results from all threads.
    results.into_iter().sum()
}

fn main() {
    // Chosen input triggers multiplication overflow when multiplied by 2.
    // For input 70000: 70000 * 2 = 140000, which exceeds u16::MAX (65535)
    let input = 70000u64;
    let total = run(input);
    println!("Total: {}", total);
}