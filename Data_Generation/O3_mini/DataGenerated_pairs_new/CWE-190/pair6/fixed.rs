--------------------------------------------------
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
struct Calculator {
    factor: i32,
}

impl Calculator {
    fn new() -> Self {
        Calculator { factor: 2 }
    }

    // Safely performs arithmetic using checked conversions and saturating arithmetic.
    fn compute(&self, base: i32, input: u32) -> i32 {
        // Attempt to convert input to i32; if it does not fit, use i32::MAX as a safe value.
        let safe_input = i32::try_from(input).unwrap_or(i32::MAX);
        // Use saturating multiplication and addition to prevent overflow.
        let addition = safe_input.saturating_mul(self.factor);
        base.saturating_add(addition)
    }
}

fn main() {
    let calc = Calculator::new();
    let balance = Arc::new(Mutex::new(10_i32));
    let calc = Arc::new(calc);
    // Use the same large number as before.
    let input_val: u32 = 0xFFFF_FFFF;
    let mut handles = vec![];
    for _ in 0..2 {
        let bal_clone = Arc::clone(&balance);
        let calc_clone = Arc::clone(&calc);
        let in_val = input_val;
        let handle = thread::spawn(move || {
            let mut num = bal_clone.lock().unwrap();
            *num = calc_clone.compute(*num, in_val);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *balance.lock().unwrap());
}
--------------------------------------------------