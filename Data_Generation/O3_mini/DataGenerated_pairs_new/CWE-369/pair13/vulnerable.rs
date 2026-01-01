use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    numerator: i32,
}

trait Calculation {
    fn compute(&self, divisor: i32) -> i32;
}

impl Calculation for Data {
    fn compute(&self, divisor: i32) -> i32 {
        // UNSAFE division without checking for zero
        unsafe {
            // If divisor is zero, this line will trigger a division-by-zero panic.
            self.numerator / divisor
        }
    }
}

fn perform(divisor: i32) -> Result<i32, &'static str> {
    let data = Arc::new(Data { numerator: 100 });
    let result = Arc::new(Mutex::new(None));

    let data_ref = Arc::clone(&data);
    let res_ref = Arc::clone(&result);

    let handle = thread::spawn(move || {
        let res = data_ref.compute(divisor);
        let mut lock = res_ref.lock().unwrap();
        *lock = Some(res);
    });

    handle.join().unwrap();
    let final_res = result.lock().unwrap();
    match *final_res {
        Some(v) => Ok(v),
        None => Err("No result computed"),
    }
}

fn main() {
    // Deliberately using a zero divisor to simulate a dangerous user input
    let user_input = 0;
    match perform(user_input) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => eprintln!("Error: {}", e),
    }
}