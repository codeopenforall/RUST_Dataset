use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io;
use std::ptr;
use std::str::FromStr;
struct Handler {
    data: Arc<Mutex<Vec<u8>>>,
}
impl Handler {
    fn new() -> Self {
        Handler {
            data: Arc::new(Mutex::new(vec![0; 10])),
        }
    }
    fn read_input(&self, filename: &str) -> io::Result<u8> {
        let content = fs::read_to_string(filename)?;
        let val = u8::from_str(content.trim()).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, e)
        })?;
        Ok(val)
    }
    unsafe fn unsafe_update(&self, index: usize, val: u8) {
        let mut vec = self.data.lock().unwrap();
        let ptr = vec.as_mut_ptr();
        if index < vec.len() {
            *ptr.add(index) = val;
        }
    }
    fn process(&self, filename: &str, index: usize) -> io::Result<u8> {
        let value = self.read_input(filename)?;
        {
            let mut data = self.data.lock().unwrap();
            data[index] = value;
        }
        unsafe {
            self.unsafe_update(index, value);
        }
        Ok(value)
    }
}
fn main() {
    let handler = Handler::new();
    let handler_clone = Handler { data: handler.data.clone() };
    let t = thread::spawn(move || {
         if let Err(e) = handler_clone.process("nonexistent.txt", 2) {
             eprintln!("Thread encountered error: {}", e);
         }
    });
    t.join().unwrap();
    if let Err(e) = handler.process("nonexistent.txt", 4) {
         eprintln!("Main encountered error: {}", e);
    }
}
