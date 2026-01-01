///////////////////////////////////////////////////////////////////////////////
// This code simulates a memory-copy operation using an unsafe FFI call.
// It incorrectly trusts an external size parameter, violating API preconditions
// for the underlying memcpy() call. When a size greater than the allocated 
// buffer length is supplied, the unsafe memcpy call may copy bytes from out-of-bound 
// memory locations, leading to undefined behavior (CWE-475).
///////////////////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;
use std::slice;

extern "C" {
    fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8;
}

trait MemoryOps {
    fn duplicate(&self, size: usize) -> Result<(), &'static str>;
}

struct DataHolder {
    store: Arc<Mutex<Vec<u8>>>,
}

impl DataHolder {
    fn new(len: usize) -> Self {
        // Allocate a vector with a predefined value.
        DataHolder { store: Arc::new(Mutex::new(vec![1u8; len])) }
    }

    fn get_ptr(&self) -> *mut u8 {
        // Return a raw mutable pointer to the beginning of the vector.
        self.store.lock().unwrap().as_mut_ptr()
    }
}

impl MemoryOps for DataHolder {
    fn duplicate(&self, size: usize) -> Result<(), &'static str> {
        let ptr = self.get_ptr();
        unsafe {
            if ptr.is_null() {
                return Err("Null pointer encountered");
            }
            // FLAW: Using memcpy without verifying that 'size' is within the allocated bounds.
            // If 'size' exceeds the vector length, undefined behavior occurs.
            memcpy(ptr.offset(0), ptr, size);
        }
        Ok(())
    }
}

fn main() {
    let holder = DataHolder::new(10);
    let shared = Arc::new(holder);
    let workers: Vec<_> = (0..2).map(|_| {
        let handle = Arc::clone(&shared);
        thread::spawn(move || {
            // Incorrect usage: Passing an invalid size (20 bytes)
            // The underlying memcpy call will process 20 bytes even if the buffer holds only 10.
            let _ = handle.duplicate(20);
        })
    }).collect();

    for worker in workers {
        let _ = worker.join();
    }
}