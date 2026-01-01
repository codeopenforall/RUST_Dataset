///////////////////////////
// Vulnerable Version
///////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Core {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Core {
    fn new(size: usize) -> Self {
        Core {
            data: Arc::new(Mutex::new(vec![0; size])),
        }
    }
    
    // Updates an element with an unsafe pointer operation.
    // Returns a Result indicating success or an error if the index is invalid.
    fn update(&self, index: usize, value: i32) -> Result<(), &'static str> {
        let lock = self.data.lock().map_err(|_| "Lock poisoned")?;
        if index >= lock.len() {
            return Err("Index out of bounds");
        }
        // Unsafe block: directly modify memory without further runtime checks.
        unsafe {
            let ptr = lock.as_ptr() as *mut i32;
            *ptr.add(index) = value;
        }
        Ok(())
    }
    
    // Spawns a thread to update the data unconditionally.
    // Any error is completely ignored.
    fn concurrent_update(&self, index: usize, value: i32) {
        let core_clone = self.data.clone();
        let _ = thread::spawn(move || {
            let lock = core_clone.lock().unwrap();
            // Unsafe block: performing unchecked memory write.
            unsafe {
                let ptr = lock.as_ptr() as *mut i32;
                // Intentionally writing without bounds verification.
                *ptr.add(index) = value;
            }
            // The result of this dangerous operation is not checked.
        });
        // The thread handle is deliberately not joined.
    }
}

fn main() {
    let engine = Core::new(10);
    
    // Concurrent update: passes an out-of-bound index (15) and ignores any error.
    engine.concurrent_update(15, 42); // Vulnerable: index 15 is beyond the vector length.
    
    // Synchronous update: result is discarded.
    let _ = engine.update(20, 99); // Vulnerable: out-of-bound update with unchecked Result.
    
    // Give spawned threads time to run.
    thread::sleep(std::time::Duration::from_millis(50));
    
    println!("Processing complete");
}