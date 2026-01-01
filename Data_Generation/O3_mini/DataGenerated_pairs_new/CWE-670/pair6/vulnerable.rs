////////////////////////////////////////////////////////////////
// Vulnerable Implementation (Always-Incorrect Control Flow)
////////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct DataProcessor {
    threshold: u32,
    data: Vec<u32>,
}

impl DataProcessor {
    fn new(threshold: u32, data: Vec<u32>) -> Self {
        Self { threshold, data }
    }

    // This method is designed to check that the provided index is valid.
    // The intended behavior is to return None if the input is below the threshold.
    // However, due to an inverted branch in the match statement, the unsafe
    // block is executed for inputs below the threshold, causing an out-of-bounds
    // read when the index is not valid.
    fn calculate(&self, index: u32) -> Option<u32> {
        // Intended logic: if index < threshold then reject access,
        // otherwise safely return the element.
        // Bug: improper match control flow logic.
        match index >= self.threshold {
            true => {
                // Safe branch: uses standard bounds-checked access.
                self.data.get(index as usize).copied()
            },
            false => {
                // Flawed branch: instead of rejecting invalid input,
                // it unsafely tries to read from the vector without proper validation.
                unsafe {
                    let ptr = self.data.as_ptr().add(index as usize);
                    Some(*ptr)
                }
            }
        }
    }
}

fn main() {
    // Using Arc and Mutex to simulate a realistic concurrent scenario.
    let processor = Arc::new(Mutex::new(DataProcessor::new(10, vec![10, 20, 30, 40, 50])));
    
    let mut handles = vec![];
    for i in 0..5 {
        let proc_clone = Arc::clone(&processor);
        handles.push(thread::spawn(move || {
            // For indices below the threshold, the function wrongly takes the unsafe branch.
            let res = proc_clone.lock().unwrap().calculate(i);
            println!("Thread {} got: {:?}", i, res);
        }));
    }
    
    for h in handles {
        h.join().unwrap();
    }

    // This call uses an index (2) that is below the threshold.
    // The expected correct behavior would be to return None,
    // but because of the incorrect control flow, an unsafe memory dereference is attempted.
    let result = processor.lock().unwrap().calculate(2);
    println!("Main result: {:?}", result);
}