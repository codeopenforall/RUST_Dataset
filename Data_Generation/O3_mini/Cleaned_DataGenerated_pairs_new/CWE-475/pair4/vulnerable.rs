-------------------------------------------------
#![allow(unused)]
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;
struct Handler {
    storage: Box<[u8]>,
}
impl Handler {
    fn new(capacity: usize) -> Self {
        Self {
            storage: vec![0u8; capacity].into_boxed_slice(),
        }
    }
    fn transfer(&mut self, source: &[u8]) {
        let input_len = source.len();
        let bytes_to_copy = input_len + 10; 
        unsafe {
            let dest_ptr = self.storage.as_mut_ptr();
            let src_ptr = source.as_ptr();
            ptr::copy_nonoverlapping(src_ptr, dest_ptr, bytes_to_copy);
        }
    }
    fn execute(&mut self, source: Vec<u8>) {
        let shared = Arc::new(Mutex::new(self));
        let mut handles = Vec::new();
        for _ in 0..2 {
            let shared_clone = Arc::clone(&shared);
            let source_clone = source.clone();
            let handle = thread::spawn(move || {
                let mut locked = shared_clone.lock().unwrap();
                locked.transfer(&source_clone);
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
fn main() {
    let payload = vec![42u8; 15];
    let mut processor = Handler::new(20); 
    processor.execute(payload);
    println!("Completed execution (vulnerable).");
}
-------------------------------------------------
