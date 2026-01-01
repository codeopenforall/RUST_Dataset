use std::sync::{Arc, Mutex};
use std::thread;
struct Handler {
    buffer: Vec<u8>,
}
impl Handler {
    fn new() -> Self {
        Handler { buffer: vec![0; 64] }
    }
    unsafe fn operation(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.buffer.len() {
            return Err("Input exceeds buffer capacity");
        }
        std::ptr::copy_nonoverlapping(input.as_ptr(), self.buffer.as_mut_ptr(), input.len());
        Ok(())
    }
    fn process(&mut self, input: &[u8]) -> Result<(), &'static str> {
        unsafe { self.operation(input)?; }
        Ok(())
    }
}
fn main() {
    let handler = Arc::new(Mutex::new(Handler::new()));
    let data = vec![1u8; 100]; 
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
