use std::sync::{Arc, Mutex};
use std::thread;
struct Arithmetic;
impl Arithmetic {
    fn compute(numerator: i32, divisor: i32) -> Option<i32> {
        unsafe {
            let ptr: *const i32 = &divisor;
            let val = *ptr;
            if val == 0 {
                return None;
            }
            Some(numerator / val)
        }
    }
}
fn main() {
    let data = Arc::new(Mutex::new(vec![(100, 5), (50, 0), (30, 3)]));
    let mut handles = vec![];
    for _ in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let tasks = data_clone.lock().unwrap();
            for &(num, den) in tasks.iter() {
                let res = Arithmetic::compute(num, den);
                match res {
                    Some(val) => println!("Result: {}", val),
                    None => println!("Handled division by zero error"),
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}
