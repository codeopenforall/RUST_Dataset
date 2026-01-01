///////////////////////////
// Fixed Version
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
    
    // Safely updates an element after proper bounds-checking.
    fn update(&self, index: usize, value: i32) -> Result<(), &'static str> {
        let mut lock = self.data.lock().map_err(|_| "Lock poisoned")?;
        if index >= lock.len() {
            return Err("Index out of bounds");
        }
        // Safe update using an unsafe block confirmed by the runtime check.
        unsafe {
            let ptr = lock.as_mut_ptr();
            *ptr.add(index) = value;
        }
        Ok(())
    }
    
    // Spawns a thread to perform the update and returns its JoinHandle.
    // The thread checks bounds and propagates errors.
    fn concurrent_update(&self, index: usize, value: i32) -> thread::JoinHandle<Result<(), &'static str>> {
        let data_clone = self.data.clone();
        thread::spawn(move || {
            let mut lock = data_clone.lock().map_err(|_| "Lock poisoned")?;
            if index >= lock.len() {
                return Err("Index out of bounds");
            }
            unsafe {
                let ptr = lock.as_mut_ptr();
                *ptr.add(index) = value;
            }
            Ok(())
        })
    }
}

fn main() {
    let engine = Core::new(10);
    
    // Perform a safe synchronous update.
    if let Err(e) = engine.update(9, 42) {
        eprintln!("update error: {}", e);
        return;
    }
    
    // Spawn a thread for a safe concurrent update and properly handle its result.
    let handle = engine.concurrent_update(8, 55);
    match handle.join() {
        Ok(Ok(())) => {},
        Ok(Err(e)) => {
            eprintln!("concurrent update error: {}", e);
            return;
        },
        Err(_) => {
            eprintln!("Thread panicked");
            return;
        }
    }
    
    // Attempt an out-of-bound update and assert that it returns an error.
    assert!(engine.update(15, 99).is_err(), "Expected error for out-of-bound update");
    
    println!("Processing complete");
}