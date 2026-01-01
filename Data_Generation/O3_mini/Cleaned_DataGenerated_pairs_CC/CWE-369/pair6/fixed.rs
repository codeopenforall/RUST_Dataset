use std::io;
use std::thread;
use std::sync::mpsc;
trait Operations {
    unsafe fn div_safe(&self, numerator: i32, denominator_ptr: *const i32) -> Result<i32, &'static str>;
}
struct Arithmetic;
impl Operations for Arithmetic {
    unsafe fn div_safe(&self, numerator: i32, denominator_ptr: *const i32) -> Result<i32, &'static str> {
        let denominator = *denominator_ptr;
        if denominator == 0 {
            return Err("Division by zero");
        }
        Ok(numerator / denominator)
    }
}
fn main() {
    let boxed = Box::new(0i32);
    let mut raw_ptr: *mut i32 = Box::into_raw(boxed);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let value: i32 = input.trim().parse().expect("Invalid integer input");
        tx.send(value).expect("Channel send failed");
    });
    let value = rx.recv().expect("Channel receive failed");
    unsafe {
        *raw_ptr = value;
    }
    let calc = Arithmetic;
    let result = unsafe { calc.div_safe(100, raw_ptr as *const i32) };
    match result {
        Ok(res) => println!("Result: {}", res),
        Err(e) => println!("Error: {}", e),
    }
    unsafe {
        Box::from_raw(raw_ptr);
    }
}
