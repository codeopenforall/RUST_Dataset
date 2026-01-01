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
        // The unsafe block uses raw pointer dereference and does not catch panics.
        // If the value is negative, an unchecked panic is triggered.
        unsafe {
            let raw = &value as *const i32;
            let result = *raw;
            if result < 0 {
                // This panic inside an unsafe block, unprotected by catch_unwind,
                // can propagate into foreign contexts or concurrent threads,
                // resulting in abnormal termination.
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
    // Spawn a new thread to run the operation.
    // No panic catching is performed, so an error in the unsafe block causes an unwinding panic.
    let join_handle = thread::spawn(move || {
        let res = handler_thread.run(value);
        res
    });
    // Unwrap the join result; if the thread panicked, this unwrap will itself panic.
    join_handle.join().unwrap()
}

fn main() {
    // Trigger the panic by passing a negative value.
    // In real-world FFI or concurrent scenarios, this unhandled panic could lead to undefined behavior.
    let output = execute_task(-1);
    println!("Result: {}", output);
}