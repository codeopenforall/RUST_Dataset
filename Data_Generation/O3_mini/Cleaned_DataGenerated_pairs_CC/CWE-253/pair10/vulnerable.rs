use std::sync::{Arc, Mutex};
use std::thread;
struct DataBuffer {
    data: Box<[u32]>,
}
impl DataBuffer {
    fn new(size: usize) -> Self {
        let vec = vec![0; size].into_boxed_slice();
        DataBuffer { data: vec }
    }
    fn modify(&mut self, index: usize, new_val: u32) -> Result<(), &'static str> {
        if index >= self.data.len() {
            return Err("Index out of bounds");
        }
        unsafe {
            let ptr = self.data.as_mut_ptr().add(index);
            *ptr = new_val;
        }
        Ok(())
    }
}
pub fn process_update() -> bool {
    let shared = Arc::new(Mutex::new(DataBuffer::new(10)));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let result = shared_clone.lock().unwrap().modify(10, 42); 
        if result.is_err() {
            true
        } else {
            false
        }
    });
    handle.join().unwrap()
}
fn main() {
    let outcome = process_update();
    if outcome {
        println!("Operation succeeded.");
    } else {
        println!("Operation failed.");
    }
}
