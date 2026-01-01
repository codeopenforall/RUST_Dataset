////////////////////////////////////////
// Corrected Example in Rust          //
// CWE-617: Reachable Assertion Fixed   //
////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    data: Arc<Mutex<Vec<u32>>>,
}

impl Engine {
    fn new() -> Self {
        // Initialize a vector with fixed, controlled values.
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        Engine {
            data: Arc::new(Mutex::new(vec)),
        }
    }
    
    fn run(&self, index: usize) {
        {
            // Verify that the index is within the valid range.
            let vec = self.data.lock().unwrap();
            if index >= vec.len() {
                eprintln!("Error: index {} out-of-bounds", index);
                return;
            }
        }
        // Spawn a thread only if the index is valid.
        let data_clone = Arc::clone(&self.data);
        let idx = index;
        let handle = thread::spawn(move || {
            let mut vec = data_clone.lock().unwrap();
            // Modify the value in a controlled manner so that the invariant remains satisfied.
            if idx < vec.len() && idx % 2 == 0 {
                // Instead of setting a high value, use a value that preserves the invariant.
                vec[idx] = 40;
            }
        });
        handle.join().unwrap();
        
        let vec = self.data.lock().unwrap();
        // Safe check prior to pointer arithmetic.
        if index < vec.len() {
            unsafe {
                let ptr = vec.as_ptr();
                let value = *ptr.add(index);
                // Instead of triggering a panic via an assertion, handle the situation gracefully.
                if value >= 50 {
                    eprintln!("Error: value {} at index {} out of acceptable range", value, index);
                    return;
                }
                println!("Value at index {} is acceptable: {}", index, value);
            }
        }
    }
}

fn main() {
    // Accept and validate input from command-line.
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        0
    };
    let engine = Engine::new();
    engine.run(input);
}