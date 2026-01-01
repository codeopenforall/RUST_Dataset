//////////////////////////////////////////////////////////////
// Vulnerable Code: Incorrect Time Conversion Calculation   //
// CWE-682: Incorrect Calculation (logic/math error)          //
// This application converts a number of seconds to milliseconds//
// using an unsafe raw pointer and concurrent thread access.  //
//////////////////////////////////////////////////////////////

#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;

struct TimeProcessor {
    unit: u64,
}

impl TimeProcessor {
    fn new() -> Self {
        // "unit" represents the conversion factor
        TimeProcessor { unit: 1000 }
    }

    fn compute(&self, secs: u64) -> u64 {
        // Vulnerable calculation: Due to operator precedence,
        // integer division occurs before multiplication.
        // For example, for secs = 500, the intended result (500 * 1000 = 500_000)
        // becomes: (500 / 1000) * 1000 = 0 * 1000 = 0.
        secs / self.unit * 1000
    }
}

fn main() {
    let processor = Arc::new(Mutex::new(TimeProcessor::new()));
    
    // Spawn a thread and use an unsafe block to simulate raw pointer manipulation.
    let proc_clone = Arc::clone(&processor);
    let handle = thread::spawn(move || {
        unsafe {
            let raw_ptr: *mut TimeProcessor = &mut *proc_clone.lock().unwrap();
            (*raw_ptr).compute(500)
        }
    });
    
    let thread_result = handle.join().unwrap();
    println!("Thread computation result: {}", thread_result);
    
    let direct_result = processor.lock().unwrap().compute(1500);
    println!("Direct computation result: {}", direct_result);
}