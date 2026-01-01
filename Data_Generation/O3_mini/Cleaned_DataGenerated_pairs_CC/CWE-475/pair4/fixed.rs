-------------------------------------------------
#![allow(unused)]
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;
struct Executor {
    storage: Box<[u8]>,
}
impl Executor {
    fn new(capacity: usize) -> Self {
        Self {
            storage: vec![0u8; capacity].into_boxed_slice(),
        }
    }
    fn transfer(&mut self, source: &[u8]) -> Result<(), &'static str> {
        let input_len = source.len();
        let desired = input_len.checked_add(10).ok_or("Length overflow")?;
        if desired > self.storage.len() || desired > source.len() {
            return Err("Requested copy length is invalid; preconditions not met.");
        }
        unsafe {
            let dest_ptr = self.storage.as_mut_ptr();
            let src_ptr = source.as_ptr();
            ptr::copy_nonoverlapping(src_ptr, dest_ptr, desired);
        }
        Ok(())
    }
    fn execute(&mut self, source: Vec<u8>) -> Result<(), &'static str> {
        let shared = Arc::new(Mutex::new(self));
        let mut handles = Vec::new();
        for _ in 0..2 {
            let shared_clone = Arc::clone(&shared);
            let source_clone = source.clone();
            let handle = thread::spawn(move || {
                let mut locked = shared_clone.lock().unwrap();
                locked.transfer(&source_clone).unwrap();
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        Ok(())
    }
}
fn main() {
    let payload = vec![42u8; 15];
    let mut runner = Executor::new(30); 
    runner.execute(payload).expect("Execution failed due to invalid preconditions.");
    println!("Completed execution (fixed).");
}
-------------------------------------------------
