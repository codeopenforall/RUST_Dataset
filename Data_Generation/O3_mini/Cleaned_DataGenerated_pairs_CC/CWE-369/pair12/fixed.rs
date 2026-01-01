use std::sync::Arc;
use std::thread;
struct Calculator;
impl Calculator {
    fn compute(&self, numerator: i32, denominator: i32) -> Result<i32, String> {
        if denominator == 0 {
            return Err(String::from("Division by zero error"));
        }
        let result = unsafe { numerator / denominator };
        Ok(result)
    }
    pub fn safe_compute(&self, numerator: i32, denominator: i32) -> Result<i32, String> {
        self.compute(numerator, denominator)
    }
}
fn main() {
    let calc = Arc::new(Calculator);
    let calc_ref = Arc::clone(&calc);
    let handle = thread::spawn(move || {
        calc_ref.compute(100, 0)
    });
    match handle.join().unwrap() {
        Ok(value) => println!("Computed result: {}", value),
        Err(err) => println!("Encountered error: {}", err),
    }
}
