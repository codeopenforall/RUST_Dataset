use std::sync::{Arc, Mutex};
use std::thread;
struct BufferWrapper {
    buffer: Vec<u8>,
}
impl BufferWrapper {
    fn new(size: usize) -> Self {
        let v = vec![0u8; size];
        BufferWrapper { buffer: v }
    }
    fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        if index >= self.buffer.len() {
            return Err("Index out-of-bounds");
        }
        self.buffer[index] = value;
        Ok(())
    }
}
fn main() {
    let wrapper = Arc::new(Mutex::new(BufferWrapper::new(10)));
    let wrapper_clone = Arc::clone(&wrapper);
    let handle = thread::spawn(move || {
        let mut lock = wrapper_clone.lock().unwrap();
        match lock.update(15, 55) {
            Ok(_) => println!("Update unexpectedly succeeded."),
            Err(e) => println!("Handled error: {}", e),
        }
    });
    handle.join().unwrap();
    println!("Execution complete: Vulnerability fixed.");
}
