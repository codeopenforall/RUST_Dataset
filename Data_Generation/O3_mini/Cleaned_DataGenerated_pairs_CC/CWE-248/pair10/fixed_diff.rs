use std::panic;
fn perform_action(divisor: i32) -> Result<i32, &'static str> {
    let result = panic::catch_unwind(|| {
        unsafe { proc.execute(divisor) }
    });
    match result {
        Ok(val) => Ok(val),
        Err(_) => Err("Operation failed: division by zero or invalid input"),
    }
        match perform_action(0) {
            Ok(result) => println!("Computed result: {}", result),
            Err(e) => println!("Error encountered: {}", e),
        }
    handle.join().expect("Thread panicked unexpectedly");
