--------------------------------------------------
#![allow(unused)]
use std::thread;

trait Task {
    fn execute(&self) -> u32;
}

struct Engine;

impl Engine {
    // This function unsafely copies data from one vector into another.
    // It deliberately passes an incorrect length (src.len() + 1) to the
    // copy function, violating the API preconditions.
    // This undefined behavior may read beyond the allocated memory.
    unsafe fn compute(&self) -> u32 {
        // Allocate a source vector with three elements.
        let src: Vec<u32> = vec![10, 20, 30];
        // Allocate a destination vector with exactly three elements.
        let mut dst: Vec<u32> = vec![0; 3];
        // POTENTIAL FLAW:
        // Calling copy_nonoverlapping with a length value exceeding the source size.
        std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len() + 1);
        // Sum the destination elements; undefined behavior may occur here.
        dst.iter().sum()
    }
}

impl Task for Engine {
    fn execute(&self) -> u32 {
        // Spawn a new thread to simulate concurrent usage.
        // The unsafe computation is wrapped in a thread.
        let handle = thread::spawn(move || unsafe { self.compute() });
        handle.join().unwrap()
    }
}

fn main() {
    let engine = Engine;
    let result = engine.execute();
    println!("Result: {}", result);
}
--------------------------------------------------