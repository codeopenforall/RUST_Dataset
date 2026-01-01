//////////////////////////////////////
// Vulnerability: Integer Underflow //
//////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Record {
    // Holds a value that will be decremented using unchecked arithmetic.
    value: u32,
}

impl Record {
    fn new(init: u32) -> Self {
        Record { value: init }
    }

    // Unsafe operation: subtracts deduction without verifying for underflow.
    // This operation is made unsafe to mimic scenarios where unchecked arithmetic combined
    // with unsafe pointer usage and concurrency might lead to real-world vulnerabilities.
    unsafe fn decrease(&mut self, deduction: u32) {
        // Manually calculate the new value using unchecked subtraction.
        // NOTE: wrapping_sub will wrap on underflow.
        self.value = self.value.wrapping_sub(deduction);
    }
}

// This function orchestrates concurrent decrements on the shared record.
// It returns the final value after all deductions.
pub fn run_process() -> u32 {
    let data = Arc::new(Mutex::new(Record::new(50)));
    let mut threads = Vec::new();
    // Spawn several threads to simulate concurrent deductions.
    for _ in 0..5 {
        let data_ref = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // Each thread subtracts a fixed deduction.
            let deduction = 20u32;
            let mut guard = data_ref.lock().unwrap();
            unsafe {
                guard.decrease(deduction);
            }
        });
        threads.push(handle);
    }
    // Wait for all threads to complete.
    for handle in threads {
        handle.join().unwrap();
    }
    // Return the final value
    let final_value = data.lock().unwrap().value;
    println!("Final value: {}", final_value);
    final_value
}

fn main() {
    // Execute the process that performs concurrent decrements.
    let _ = run_process();
}