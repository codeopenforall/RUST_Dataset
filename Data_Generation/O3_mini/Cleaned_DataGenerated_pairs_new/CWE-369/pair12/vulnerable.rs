use std::sync::Arc;
use std::thread;
use std::panic;
struct Calculator;
impl Calculator {
    fn compute(&self, numerator: i32, denominator: i32) -> i32 {
        unsafe {
            numerator / denominator  
        }
    }
    pub fn safe_compute(&self, numerator: i32, denominator: i32) -> Result<i32, String> {
        let res = panic::catch_unwind(|| self.compute(numerator, denominator));
        match res {
            Ok(val) => Ok(val),
            Err(_) => Err(String::from("Panic occurred")),
        }
    }
}
fn main() {
    let calc = Arc::new(Calculator);
    let calc_ref = Arc::clone(&calc);
    let handle = thread::spawn(move || {
        calc_ref.compute(100, 0)
    });
    let result = handle.join().unwrap();
    println!("Computed result: {}", result);
}
