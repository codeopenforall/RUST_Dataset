////////////////////////////////////////////////////////////////
// Corrected Implementation (Proper Control Flow)
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

    // Correctly verifies the input index. If the index is below the threshold,
    // it returns None. Otherwise, it performs a safe lookup.
    fn calculate(&self, index: u32) -> Option<u32> {
        if index < self.threshold {
            return None;
        }
        self.data.get(index as usize).copied()
    }
}

fn main() {
    // Adjusted vector length to ensure valid lookups on the safe branch.
    let processor = Arc::new(Mutex::new(DataProcessor::new(10, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110])));
    
    let mut handles = vec![];
    for i in 0..5 {
        let proc_clone = Arc::clone(&processor);
        handles.push(thread::spawn(move || {
            // For indices below the threshold, the corrected flow returns None,
            // ensuring a safe API even in concurrent settings.
            let res = proc_clone.lock().unwrap().calculate(i);
            println!("Thread {} got: {:?}", i, res);
        }));
    }
    
    for h in handles {
        h.join().unwrap();
    }

    // This call uses an index within the valid range.
    let result = processor.lock().unwrap().calculate(10);
    println!("Main result: {:?}", result);
}