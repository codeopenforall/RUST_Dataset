use std::sync::Arc;
use std::thread;

struct Calculator;

impl Calculator {
    // Performs multiplication without overflow check using an unsafe block.
    fn compute(&self, n: u32) -> Result<u32, &'static str> {
        unsafe {
            // The multiplication is carried out using a cast which does not check for overflow.
            let result = ((n as u64) * (100000 as u64)) as u32;
            Ok(result)
        }
    }
}

fn main() {
    let calc = Calculator;
    let input = 50000u32;
    let shared_calc = Arc::new(calc);
    let calc_clone = Arc::clone(&shared_calc);

    let handler = thread::spawn(move || {
        // This computation may overflow and produce an unexpected wrapped value.
        calc_clone.compute(input)
    });

    match handler.join().unwrap() {
        Ok(val) => {
            println!("Result is: {}", val);
        },
        Err(msg) => {
            println!("Error: {}", msg);
        }
    }
}