use std::panic;
    unsafe fn transform(&self, a: i32, b: i32) -> i32;
    unsafe fn transform(&self, a: i32, b: i32) -> i32 {
        a / factor
fn compute(a: i32, b: i32) -> i32 {
    let res = panic::catch_unwind(|| compute(a, b));
    match res {
        Ok(val) => Ok(val),
        Err(_) => Err("Division by zero occurred"),
    }
    let (tx, rx) = mpsc::channel();
    let _ = rx.recv().unwrap();
