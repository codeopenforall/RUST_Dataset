use std::sync::Arc;
use std::thread;
struct Calculator;
impl Calculator {
    fn compute(&self, n: u32) -> Result<u32, &'static str> {
        n.checked_mul(100000).ok_or("overflow")
    }
}
fn main() {
    let calc = Calculator;
    let input = 50000u32;
    let shared_calc = Arc::new(calc);
    let calc_clone = Arc::clone(&shared_calc);
    let handler = thread::spawn(move || {
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
