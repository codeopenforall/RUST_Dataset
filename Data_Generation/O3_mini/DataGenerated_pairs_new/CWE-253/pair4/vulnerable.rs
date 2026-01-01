/* Vulnerable code: Incorrect check of function return value utilizing unsafe operations and concurrency */
use std::sync::{Arc, Mutex};
use std::thread;

struct Handler {
    buffer: Vec<u8>,
}

impl Handler {
    fn new() -> Self {
        Handler { buffer: vec![0; 64] }
    }

    // Performs a low-level copy with unsafe pointer usage.
    unsafe fn operation(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.buffer.len() {
            return Err("Input exceeds buffer capacity");
        }
        std::ptr::copy_nonoverlapping(input.as_ptr(), self.buffer.as_mut_ptr(), input.len());
        Ok(())
    }

    // Processes the input and returns the result.
    // Vulnerability: The error returned by operation() is misinterpreted as success.
    fn process(&mut self, input: &[u8]) -> Result<(), &'static str> {
        let result = unsafe { self.operation(input) };
        // Incorrectly treating an error as successful operation.
        if result.is_err() {
            return Ok(());  // BUG: Returning success even on error.
        }
        result
    }
}

fn main() {
    let handler = Arc::new(Mutex::new(Handler::new()));
    let data = vec![1u8; 100]; // Oversized input to trigger error.
    let thread_handle = {
        let cloned = Arc::clone(&handler);
        thread::spawn(move || {
            let mut instance = cloned.lock().unwrap();
            let _ = instance.process(&data);
        })
    };

    thread_handle.join().unwrap();
    println!("Execution finished");
}