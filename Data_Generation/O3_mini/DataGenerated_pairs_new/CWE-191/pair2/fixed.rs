//////////////////////////////////////////////
// Corrected: Preventing Integer Underflow  //
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Record {
    // Holds a value that will be decremented safely.
    value: u32,
}

impl Record {
    fn new(init: u32) -> Self {
        Record { value: init }
    }

    // Safely decrease the value only if the deduction does not exceed the current value.
    // Returns Ok(()) if the operation is successful or an error if an underflow would occur.
    fn decrease(&mut self, deduction: u32) -> Result<(), &'static str> {
        if deduction > self.value {
            Err("Integer underflow prevented: deduction exceeds current value")
        } else {
            self.value -= deduction;
            Ok(())
        }
    }
}

// This function orchestrates concurrent safe decrements on the shared record.
// It returns the final value after processing. Threads which would cause an underflow do not alter the value.
pub fn run_process() -> u32 {
    let data = Arc::new(Mutex::new(Record::new(50)));
    let mut threads = Vec::new();
    // Spawn several threads to simulate concurrent operations.
    for _ in 0..5 {
        let data_ref = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let deduction = 20u32;
            let mut guard = data_ref.lock().unwrap();
            // Only subtract if it is safe; ignore error if underflow would occur.
            let _ = guard.decrease(deduction);
        });
        threads.push(handle);
    }
    // Wait for all threads to complete.
    for handle in threads {
        handle.join().unwrap();
    }
    // Return the final value.
    let final_value = data.lock().unwrap().value;
    println!("Final value: {}", final_value);
    final_value
}

fn main() {
    // Execute the safe concurrent process.
    let _ = run_process();
}