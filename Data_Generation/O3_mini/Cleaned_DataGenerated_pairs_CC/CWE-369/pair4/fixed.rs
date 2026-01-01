use std::sync::{Arc, Mutex};
use std::thread;
pub struct Calculator;
impl Calculator {
    pub unsafe fn compute(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            return Err("Division by zero");
        }
        let a_ptr: *const i32 = &a;
        let b_ptr: *const i32 = &b;
        let numerator = *a_ptr;
        let divisor = *b_ptr;
        Ok(numerator / divisor)
    }
}
pub unsafe fn perform_division(a: i32, b: i32) -> i32 {
    let calc = Calculator;
    match calc.compute(a, b) {
        Ok(val) => val,
        Err(err) => {
            println!("Error encountered: {}", err);
            0  
        }
    }
}
fn main() {
    let calc = Arc::new(Calculator);
    let data = Arc::new(Mutex::new(vec![(10, 2), (10, 0)])); 
    let mut handles = Vec::new();
    for _ in 0..data.lock().unwrap().len() {
        let calc_clone = calc.clone();
        let data_clone = data.clone();
        let handle = thread::spawn(move || {
            let pair = data_clone.lock().unwrap().pop().unwrap();
            unsafe {
                let res = match calc_clone.compute(pair.0, pair.1) {
                    Ok(val) => val,
                    Err(err) => {
                        println!("Error encountered: {}", err);
                        0
                    }
                };
                println!("Computed result: {}", res);
                res
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}
