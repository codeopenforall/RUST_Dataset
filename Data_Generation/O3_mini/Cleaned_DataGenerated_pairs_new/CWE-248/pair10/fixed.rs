use std::thread;
use std::panic;
trait Operation {
    unsafe fn execute(&self, divisor: i32) -> i32;
}
struct Processor {
    base: i32,
}
impl Operation for Processor {
    unsafe fn execute(&self, divisor: i32) -> i32 {
        self.base / divisor
    }
}
fn perform_action(divisor: i32) -> Result<i32, &'static str> {
    let proc = Processor { base: 100 };
    let result = panic::catch_unwind(|| {
        unsafe { proc.execute(divisor) }
    });
    match result {
        Ok(val) => Ok(val),
        Err(_) => Err("Operation failed: division by zero or invalid input"),
    }
}
fn main() {
    let handle = thread::spawn(|| {
        match perform_action(0) {
            Ok(result) => println!("Computed result: {}", result),
            Err(e) => println!("Error encountered: {}", e),
        }
    });
    handle.join().expect("Thread panicked unexpectedly");
}
