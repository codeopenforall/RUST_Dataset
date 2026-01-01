///////////////////////////////////////////////////////
// Rust program exhibiting a divide‐by‐zero vulnerability.
// This program uses unsafe blocks, concurrency, and smart pointers
// to simulate a realistic scenario where unchecked arithmetic with
// user‐defined divisors may lead to a runtime panic.
///////////////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

struct Computation {
    numerator: i32,
    denominator: i32,
}

impl Computation {
    // This method unsafely reads the numerator through a raw pointer
    // and performs division without validating the denominator.
    fn execute(&self) -> i32 {
        unsafe {
            let ptr = &self.numerator as *const i32;
            let num = *ptr;   // read numerator unsafely
            // POTENTIAL DIVIDE-BY-ZERO if self.denominator is zero
            num / self.denominator
        }
    }
}

fn main() {
    // In a real-world scenario the denominator may come from user input.
    // Here it is purposefully set to 0 to trigger the divide-by-zero vulnerability.
    let instance = Arc::new(Computation { numerator: 100, denominator: 0 });
    let instance_clone = Arc::clone(&instance);
    let handler = thread::spawn(move || {
        instance_clone.execute()
    });
    // The join will trigger a panic if the thread panicked due to division-by-zero.
    let result = handler.join().expect("Thread panicked due to divide-by-zero");
    println!("Result: {}", result);
}