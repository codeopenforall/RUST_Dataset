use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
struct Handler {
    buffer: *const u8,
    size: usize,
}
impl Handler {
    fn new(data: &[u8]) -> Self {
        Handler {
            buffer: data.as_ptr(),
            size: data.len(),
        }
    }
    unsafe fn operate(&self) -> Vec<u8> {
        if self.size == 0 {
            return Vec::new();
        }
        let copy_len = self.size - 1; 
        let mut output = vec![0u8; copy_len];
        let offset_ptr = self.buffer.offset(1);  
        ptr::copy_nonoverlapping(offset_ptr, output.as_mut_ptr(), copy_len);
        output
    }
}
fn main() {
    let data = vec![10, 20, 30, 40, 50];
    let handler = Handler::new(&data);
    let shared_val = Arc::new(Mutex::new(0));
    let shared_copy = Arc::clone(&shared_val);
    let joiner = thread::spawn(move || {
         let mut val = shared_copy.lock().unwrap();
         *val += 1;
    });
    unsafe {
         let result = handler.operate();
         println!("Output: {:?}", result);
    }
    joiner.join().unwrap();
}
