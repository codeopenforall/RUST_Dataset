use std::sync::{Arc, Mutex};
use std::thread;
struct Data {
    numerator: i32,
}
trait Calculation {
    fn compute(&self, divisor: i32) -> Result<i32, &'static str>;
}
impl Calculation for Data {
    fn compute(&self, divisor: i32) -> Result<i32, &'static str> {
        if divisor == 0 {
            return Err("Attempted division by zero");
        }
        unsafe {
            Ok(self.numerator / divisor)
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
        Some(Ok(v)) => Ok(v),
        Some(Err(e)) => Err(e),
        None => Err("No result computed"),
    }
}
fn main() {
    let user_input = 0;
    match perform(user_input) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => eprintln!("Error: {}", e),
    }
}
