///////////////////////////////
// Vulnerable Implementation //
///////////////////////////////
use std::sync::Arc;
use std::thread;

struct Engine {
    base: i32,
    multiplier: i32,
    divisor: i32,
}

impl Engine {
    // This function performs a calculation using unsafe code.
    // It incorrectly applies integer division before multiplication,
    // and uses an unsafe pointer read.
    fn process(&self) -> i32 {
        // The intention was to compute (base * multiplier) / divisor.
        // However, the unsafe block forces the division on multiplier first.
        unsafe {
            // Acquire a pointer to multiplier.
            let ptr: *const i32 = &self.multiplier;
            // Incorrect calculation: due to integer division precedence,
            // the multiplier is divided by divisor before multiplying by base.
            let factor = *ptr / self.divisor;  // Vulnerable operation: misordered math.
            self.base * factor
        }
    }
}

fn main() {
    // Using Arc to share the instance across threads.
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