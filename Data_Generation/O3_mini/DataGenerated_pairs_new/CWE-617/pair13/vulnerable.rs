////////////////////////////////////
// Vulnerability Example in Rust  //
// CWE-617: Reachable Assertion   //
////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    data: Arc<Mutex<Vec<u32>>>,
}

impl Engine {
    fn new() -> Self {
        // Initialize a vector with fixed values.
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        Engine {
            data: Arc::new(Mutex::new(vec)),
        }
    }
    
    fn run(&self, index: usize) {
        // Spawn a thread that may modify the element at a given index.
        // If the index is even and within bounds, it sets the value to 100.
        let data_clone = Arc::clone(&self.data);
        let idx = index;
        let handle = thread::spawn(move || {
            let mut vec = data_clone.lock().unwrap();
            if idx < vec.len() && idx % 2 == 0 {
                // Unsafe modification: the value is set to a number that will trigger the assertion.
                vec[idx] = 100;
            }
        });
        handle.join().unwrap();
        
        let vec = self.data.lock().unwrap();
        unsafe {
            // Direct pointer arithmetic with attacker-controlled index.
            // No proper boundary checking and invariant validation is done.
            let ptr = vec.as_ptr();
            let value = *ptr.add(index);
            // Vulnerability: Reachable assertion that may be triggered if 'value' is high.
            assert!(value < 50, "Assertion triggered by unexpected state");
        }
    }
}

fn main() {
    // Accept input from command-line. The attacker-controlled 'index' is parsed here.
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(0)
    } else {
        0
    };
    let engine = Engine::new();
    engine.run(input);
}