--------------------------------------------------
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
    fn compute(&self, base: i32, input: u32) -> i32 {
        unsafe {
            let addition = (input as i32) * self.factor;
            base + addition
        }
    }
}
fn main() {
    let calc = Calculator::new();
    let balance = Arc::new(Mutex::new(10_i32));
    let calc = Arc::new(calc);
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
