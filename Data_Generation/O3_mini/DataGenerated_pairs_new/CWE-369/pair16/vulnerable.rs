use std::sync::Arc;
use std::thread;

struct Processor;

impl Processor {
    // This function returns an Option<i32> but does not validate the divisor.
    // In an unsafe block it directly performs the division; if the divisor is zero,
    // a runtime panic (or UB in unsafe numeric contexts) will occur.
    fn operate(&self, num: i32, den: i32) -> Option<i32> {
        unsafe {
            // Vulnerable operation: no check for zero divisor.
            // When den == 0, this will panic at runtime.
            Some(num / den)
        }
    }
}

fn main() {
    let proc = Arc::new(Processor);
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || {
        // Using a divisor value of zero triggers the vulnerability.
        let result = proc_clone.operate(100, 0);
        println!("Result: {:?}", result);
    });
    let _ = handle.join();
}