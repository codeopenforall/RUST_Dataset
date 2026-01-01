//////////////////////////////////////////////////////////////
// Fixed Code: Correct Time Conversion Calculation            //
// CWE-682: Incorrect Calculation (logic/math error resolved)   //
// This application now correctly converts seconds to milliseconds//
// safely and without employing unnecessary unsafe blocks.      //
//////////////////////////////////////////////////////////////

#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;

struct TimeProcessor {
    unit: u64,
}

impl TimeProcessor {
    fn new() -> Self {
        TimeProcessor { unit: 1000 }
    }

    fn compute(&self, secs: u64) -> u64 {
        // Correct calculation: Multiply seconds by the conversion factor.
        secs * self.unit
    }
}

fn main() {
    let processor = Arc::new(Mutex::new(TimeProcessor::new()));
    
    // Spawn a thread; safe access via the Mutex (unsafe block removed).
    let proc_clone = Arc::clone(&processor);
    let handle = thread::spawn(move || {
        proc_clone.lock().unwrap().compute(500)
    });
    
    let thread_result = handle.join().unwrap();
    println!("Thread computation result: {}", thread_result);
    
    let direct_result = processor.lock().unwrap().compute(1500);
    println!("Direct computation result: {}", direct_result);
}