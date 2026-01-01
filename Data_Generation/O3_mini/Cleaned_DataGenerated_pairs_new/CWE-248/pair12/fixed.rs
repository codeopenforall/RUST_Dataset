use std::panic;
use std::sync::{Arc, Mutex};
use std::thread;
trait Processor {
    fn run(&self, value: i32) -> Result<i32, String>;
}
struct Handler {
    data: Arc<Mutex<i32>>,
}
impl Processor for Handler {
    fn run(&self, value: i32) -> Result<i32, String> {
        let result = panic::catch_unwind(|| unsafe {
            let raw = &value as *const i32;
            let res = *raw;
            if res < 0 {
                panic!("Simulated panic in unsafe block: negative value");
            }
            res
        });
        match result {
            Ok(val) => Ok(val),
            Err(_) => Err("Caught panic in unsafe block".to_string()),
        }
    }
}
fn execute_task(value: i32) -> Result<i32, String> {
    let handler = Handler {
        data: Arc::new(Mutex::new(value)),
    };
    let shared_handler = Arc::new(handler);
    let handler_thread = shared_handler.clone();
    let join_handle = thread::spawn(move || {
        handler_thread.run(value)
    });
    match join_handle.join() {
        Ok(res) => res,
        Err(_) => Err("Thread panicked unexpectedly".to_string()),
    }
}
fn main() {
    match execute_task(-1) {
        Ok(output) => println!("Result: {}", output),
        Err(err) => println!("Handled error: {}", err),
    }
}
