use std::sync::{Arc, Mutex};
use std::thread;
struct Buffer {
    data: Vec<u8>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        Buffer { data: vec![0; size] }
    }
    pub fn update(&mut self, index: usize, val: u8) -> Result<(), &'static str> {
        if index < self.data.len() {           
            unsafe {
                let ptr = self.data.as_mut_ptr().add(index);
                *ptr = val;
            }
            Ok(())
        } else {
            Err("index out-of-bound")
        }
    }
    pub fn read(&self, index: usize) -> Result<u8, &'static str> {
        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_ptr().add(index);
                Ok(*ptr)
            }
        } else {
            Err("index out-of-bound")
        }
    }
}
fn main() {
    let buf = Arc::new(Mutex::new(Buffer::new(5)));
    let mut handles = vec![];
    for i in 0..10 {
        let buf_clone = Arc::clone(&buf);
        let handle = thread::spawn(move || {
            let mut locked = buf_clone.lock().unwrap();
            let _ = locked.update(i, i as u8);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Execution completed in fixed build.");
}
