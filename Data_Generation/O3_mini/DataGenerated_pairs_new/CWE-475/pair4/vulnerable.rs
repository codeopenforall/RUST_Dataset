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

    // This function improperly calculates the number of bytes to copy and violates
    // the preconditions of copy_nonoverlapping by copying more bytes than available.
    fn transfer(&mut self, source: &[u8]) {
        let input_len = source.len();
        let bytes_to_copy = input_len + 10; // Unsafe: may exceed the bounds of destination or source.
        unsafe {
            let dest_ptr = self.storage.as_mut_ptr();
            let src_ptr = source.as_ptr();
            // Violates API preconditions when bytes_to_copy > self.storage.len() or > source.len()
            ptr::copy_nonoverlapping(src_ptr, dest_ptr, bytes_to_copy);
        }
    }

    // Runs the transfer concurrently on two threads.
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
    // Triggering input: source vector length is 15 while transfer copies 25 bytes.
    let payload = vec![42u8; 15];
    let mut processor = Handler::new(20); // Buffer too small to accommodate the copy.
    processor.execute(payload);
    println!("Completed execution (vulnerable).");
}
-------------------------------------------------