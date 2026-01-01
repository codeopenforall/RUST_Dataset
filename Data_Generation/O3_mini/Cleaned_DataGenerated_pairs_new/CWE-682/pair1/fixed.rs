use std::sync::{Arc, Mutex};
use std::thread;
struct Calculation {
    value: i64,
}
impl Calculation {
    fn compute(&self, factor: i64) -> i64 {
        unsafe {
            let ptr: *const i64 = &self.value;
            (*ptr + factor) / 2
        }
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(Calculation { value: 10 }));
    let operation_factor = 5; 
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let result;
        {
            let calc = shared_clone.lock().unwrap();
            result = calc.compute(operation_factor);
        }
        result
    });
    let final_result = handle.join().unwrap();
    println!("Computed result: {}", final_result);
}
