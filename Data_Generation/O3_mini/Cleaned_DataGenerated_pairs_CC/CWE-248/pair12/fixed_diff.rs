use std::panic;
    fn run(&self, value: i32) -> Result<i32, String>;
    fn run(&self, value: i32) -> Result<i32, String> {
        let result = panic::catch_unwind(|| unsafe {
            let res = *raw;
            if res < 0 {
            res
        });
        match result {
            Ok(val) => Ok(val),
            Err(_) => Err("Caught panic in unsafe block".to_string()),
fn execute_task(value: i32) -> Result<i32, String> {
        handler_thread.run(value)
    match join_handle.join() {
        Ok(res) => res,
        Err(_) => Err("Thread panicked unexpectedly".to_string()),
    }
    match execute_task(-1) {
        Ok(output) => println!("Result: {}", output),
        Err(err) => println!("Handled error: {}", err),
    }
