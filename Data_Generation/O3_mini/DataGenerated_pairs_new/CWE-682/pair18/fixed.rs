/////////////////////////////
// Corrected Implementation//
/////////////////////////////
use std::sync::Arc;
use std::thread;

struct Engine {
    base: i32,
    multiplier: i32,
    divisor: i32,
}

impl Engine {
    // This method now correctly computes (base * multiplier) / divisor.
    fn process(&self) -> i32 {
        // Correct operator precedence: perform multiplication first.
        (self.base * self.multiplier) / self.divisor
    }
}

fn main() {
    // Using Arc to safely share the object across threads.
    let engine = Arc::new(Engine { base: 100, multiplier: 3, divisor: 12 });
    let mut handles = vec![];
    // Spawn multiple threads to simulate concurrent usage.
    for _ in 0..4 {
        let eng = Arc::clone(&engine);
        handles.push(thread::spawn(move || eng.process()));
    }
    let mut total = 0;
    for h in handles {
        total += h.join().expect("Thread panicked");
    }
    println!("Result: {}", total);
}