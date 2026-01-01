use std::sync::Arc;
use std::thread;

struct Processor {
    factor: u32,
}

impl Processor {
    // Performs safe multiplication with explicit overflow detection.
    fn calculate(&self, input: u32) -> Result<u32, &'static str> {
        let prod: u64 = (input as u64).wrapping_mul(self.factor as u64);
        if prod > u32::MAX as u64 {
            Err("Overflow detected")
        } else {
            Ok(prod as u32)
        }
    }

    fn check(&self, input: u32) -> Result<u32, &'static str> {
        self.calculate(input)
    }
}

fn main() {
    // Create a shared instance wrapped in Arc for concurrent access.
    let processor = Arc::new(Processor { factor: 70000 });
    let input = 70000; // Same triggering input

    // Spawn threads to simulate concurrent execution.
    let mut handles = Vec::new();
    for _ in 0..4 {
        let proc_clone = Arc::clone(&processor);
        handles.push(thread::spawn(move || {
            match proc_clone.check(input) {
                Ok(result) => println!("Result: {}", result),
                Err(e) => println!("Error: {}", e),
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}