#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct OperationResult {
    is_timeout: bool,
    is_cancelled: bool,
    value: i32,
}
trait Processor {
    fn process(&self) -> Result<i32, &'static str>;
}
struct Worker {
    data: i32,
}
impl Processor for Worker {
    fn process(&self) -> Result<i32, &'static str> {
        let op = simulate_operation();
        let mut local = op.value;
        let raw_ptr: *mut i32 = &mut local;
        if op.is_timeout {
            unsafe {
                *raw_ptr = -1;
            }
            return Err("Timeout occurred");
        }
        let shared = Arc::new(Mutex::new(local));
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let mut val = shared_clone.lock().unwrap();
            *val += 1;
        });
        handle.join().unwrap();
        let final_val = *shared.lock().unwrap();
        Ok(final_val)
    }
}
fn simulate_operation() -> OperationResult {
    OperationResult { is_timeout: true, is_cancelled: false, value: 42 }
}
fn main() {
    let worker = Worker { data: 10 };
    match worker.process() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
