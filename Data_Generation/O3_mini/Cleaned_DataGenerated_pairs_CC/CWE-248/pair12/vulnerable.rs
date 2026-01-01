use std::sync::{Arc, Mutex};
use std::thread;
trait Processor {
    fn run(&self, value: i32) -> i32;
}
struct Handler {
    data: Arc<Mutex<i32>>,
}
impl Processor for Handler {
    fn run(&self, value: i32) -> i32 {
        unsafe {
            let raw = &value as *const i32;
            let result = *raw;
            if result < 0 {
                panic!("Simulated panic in unsafe block: negative value");
            }
            result
        }
    }
}
fn execute_task(value: i32) -> i32 {
    let handler = Handler {
        data: Arc::new(Mutex::new(value)),
    };
    let shared_handler = Arc::new(handler);
    let handler_thread = shared_handler.clone();
    let join_handle = thread::spawn(move || {
        let res = handler_thread.run(value);
        res
    });
    join_handle.join().unwrap()
}
fn main() {
    let output = execute_task(-1);
    println!("Result: {}", output);
}
