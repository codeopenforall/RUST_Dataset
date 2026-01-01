///////////////////////////////////////////////////////
// Revised Rust program that fixes the divide‐by‐zero issue.
// The program retains unsafe blocks and concurrency to reflect realistic usage,
// but now validates the arithmetic input to prevent division by zero.
///////////////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Computation {
    numerator: i32,
    denominator: i32,
}

impl Computation {
    // This method checks for a zero denominator before performing the division.
    // If the denominator is zero, it returns an error instead of panicking.
    fn execute(&self) -> Result<i32, &'static str> {
        if self.denominator == 0 {
            return Err("Error: Division by zero encountered");
        }
        unsafe {
            let ptr = &self.numerator as *const i32;
            let num = *ptr;   // read numerator unsafely
            Ok(num / self.denominator)
        }
    }
}

fn main() {
    // The denominator is set to 0 to illustrate handling the error gracefully.
    let instance = Arc::new(Computation { numerator: 100, denominator: 0 });
    let instance_clone = Arc::clone(&instance);
    let handler = thread::spawn(move || {
        instance_clone.execute()
    });
    let result = handler.join().expect("Thread panicked unexpectedly");
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("{}", err)
    }
}