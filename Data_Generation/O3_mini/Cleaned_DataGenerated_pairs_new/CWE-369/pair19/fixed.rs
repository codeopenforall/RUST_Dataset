use std::sync::Arc;
use std::thread;
struct Calculator;
impl Calculator {
    fn compute(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            return Err("Division by zero");
        }
        Ok(a / b)
    }
}
fn main() {
    let calc = Arc::new(Calculator);
    let calc_clone = Arc::clone(&calc);
    let join_handle = thread::spawn(move || {
        calc_clone.compute(100, 0)
    });
    let res = join_handle.join().expect("Thread panicked unexpectedly.");
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
