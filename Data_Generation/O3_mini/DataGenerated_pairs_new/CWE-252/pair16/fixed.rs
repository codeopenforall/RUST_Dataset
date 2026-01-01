//////////////////////////////////////////////////////////////
// This is the corrected version where the return value is
// properly checked in each thread. Tasks executing an update will
// panic immediately if an operation fails, ensuring that errors are not
// silently ignored. The overall structure remains the same but now
// enforces proper error handling.
//////////////////////////////////////////////////////////////

use std::cell::UnsafeCell;
use std::sync::{Arc, Mutex};
use std::thread;

pub trait Task {
    fn execute(&self) -> Result<(), &'static str>;
}

pub struct Worker {
    manager: Arc<Manager>,
    val: u8,
}

impl Task for Worker {
    fn execute(&self) -> Result<(), &'static str> {
        // Call the update which may fail.
        let res = self.manager.append(self.val);
        // Perform an unsafe operation to mimic low-level system calls.
        unsafe {
            let ptr = self.manager.raw_data();
            let _ = *ptr;
        }
        res
    }
}

pub struct Manager {
    data: Mutex<Vec<u8>>,
    capacity: usize,
}

impl Manager {
    pub fn new(cap: usize) -> Self {
        Self {
            data: Mutex::new(Vec::with_capacity(cap)),
            capacity: cap,
        }
    }

    // Append a byte to the stored data; error if capacity exceeded.
    pub fn append(&self, byte: u8) -> Result<(), &'static str> {
        let mut guard = self.data.lock().unwrap();
        if guard.len() >= self.capacity {
            return Err("capacity exceeded");
        }
        guard.push(byte);
        Ok(())
    }

    // Provide a raw pointer for complex operations (unsafe).
    pub unsafe fn raw_data(&self) -> *const u8 {
        self.data.lock().unwrap().as_ptr()
    }

    pub fn len(&self) -> usize {
        let guard = self.data.lock().unwrap();
        guard.len()
    }
}

fn run() {
    let mgr = Arc::new(Manager::new(10));
    let mut threads = vec![];

    // Spawn 20 threads; now each thread checks the result of execute().
    for i in 0..20 {
        let worker = Worker {
            manager: mgr.clone(),
            val: i as u8,
        };
        let handle = thread::spawn(move || {
            // Properly handle the result.
            match worker.execute() {
                Ok(()) => {}
                Err(err) => {
                    // Immediately panic on error to avoid ignoring failure.
                    panic!("Operation failed: {}", err);
                }
            }
        });
        threads.push(handle);
    }

    // Gather thread results; if any thread panicked, this will propagate.
    for handle in threads {
        // Using join with error handling.
        handle.join().expect("A thread panicked");
    }

    println!("Final data length: {}", mgr.len());
}

fn main() {
    run();
}