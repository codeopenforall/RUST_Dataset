use std::thread;
use std::panic;
unsafe fn compute(val: i32) -> i32 {
    if val < 0 {
        panic!("Invalid value: negative input not allowed");
    }
    val * 2
}
fn run_task(input: i32) -> Result<i32, String> {
    let handle = thread::spawn(move || {
        panic::catch_unwind(|| unsafe { compute(input) })
    });
    match handle.join() {
        Ok(result) => match result {
            Ok(val) => Ok(val),
            Err(_) => Err("Caught panic in worker thread".to_string()),
        },
        Err(_) => Err("Worker thread panicked unexpectedly".to_string()),
    }
}
fn main() {
    match run_task(-1) {
        Ok(val) => println!("Computation succeeded: {}", val),
        Err(err) => println!("Computation failed gracefully: {}", err),
    }
}
