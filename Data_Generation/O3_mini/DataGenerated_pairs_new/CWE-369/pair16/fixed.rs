use std::sync::Arc;
use std::thread;

struct Processor;

impl Processor {
    // This function returns an Option<i32> and safely checks the divisor.
    // If the divisor is zero, it returns None; otherwise it returns the division result.
    fn operate(&self, num: i32, den: i32) -> Option<i32> {
        if den == 0 {
            // Safely handle division by zero.
            None
        } else {
            // Safe division now that the divisor is checked.
            Some(num / den)
        }
    }
}

fn main() {
    let proc = Arc::new(Processor);
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || {
        // The fixed code handles a zero divisor by returning None instead of panicking.
        match proc_clone.operate(100, 0) {
            Some(result) => println!("Result: {}", result),
            None => println!("Error: Division by zero avoided"),
        }
    });
    let _ = handle.join();
}