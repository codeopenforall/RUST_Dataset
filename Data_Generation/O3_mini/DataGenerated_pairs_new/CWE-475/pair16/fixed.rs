--------------------------------------------------
#![allow(unused)]
use std::thread;

trait Task {
    fn execute(&self) -> u32;
}

struct Engine;

impl Engine {
    // This corrected function safely copies data by ensuring that the number of elements
    // copied exactly matches the source vector length. This guarantees that no out-of-bound
    // memory is accessed.
    unsafe fn compute_safe(&self) -> u32 {
        let src: Vec<u32> = vec![10, 20, 30];
        let mut dst: Vec<u32> = vec![0; 3];
        // FIX: Use the correct length (src.len()) in the call.
        std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len());
        dst.iter().sum()
    }
}

impl Task for Engine {
    fn execute(&self) -> u32 {
        let handle = thread::spawn(move || unsafe { self.compute_safe() });
        handle.join().unwrap()
    }
}

fn main() {
    let engine = Engine;
    let result = engine.execute();
    println!("Result: {}", result);
}
--------------------------------------------------