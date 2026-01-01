//////////////////////////////////////////////////////////////
// This Rust program demonstrates a vulnerability due to an
// unchecked return value. A shared manager holds a vector
// with a fixed capacity. Multiple threads try to append data
// via a task interface. The append operation returns a Result,
// but in the thread closure the result is discarded. As a
// consequence, errors (e.g. “capacity exceeded”) are ignored,
// which in a real system could lead to logical inconsistencies.
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
        // Call the update function that may fail if capacity is exceeded.
        let res = self.manager.append(self.val);
        // An additional unsafe part to mimic low-level manipulation.
        unsafe {
            let ptr = self.manager.raw_data();
            // Arbitrary unsafe read to simulate complexity.
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

    // Append a byte if there is capacity; return Err if not.
    pub fn append(&self, byte: u8) -> Result<(), &'static str> {
        let mut guard = self.data.lock().unwrap();
        if guard.len() >= self.capacity {
            return Err("capacity exceeded");
        }
        guard.push(byte);
        Ok(())
    }

    // Unsafe accessor that returns a raw pointer to the vector's data.
    pub unsafe fn raw_data(&self) -> *const u8 {
        self.data.lock().unwrap().as_ptr()
    }

    pub fn len(&self) -> usize {
        let guard = self.data.lock().unwrap();
        guard.len()
    }
}

fn run() {
    // This manager has a fixed capacity of 10 bytes.
    let mgr = Arc::new(Manager::new(10));
    let mut threads = vec![];

    // Spawn 20 threads that each attempt to append a value.
    // Vulnerability: The return value of execute() is not checked.
    for i in 0..20 {
        let worker = Worker {
            manager: mgr.clone(),
            val: i as u8,
        };
        let handle = thread::spawn(move || {
            // Critical vulnerability: Discarding the Result may hide errors.
            let _ = worker.execute();
        });
        threads.push(handle);
    }

    // Wait for all threads to complete.
    for handle in threads {
        handle.join().unwrap();
    }

    // Final state is printed, but errors during appending are ignored.
    println!("Final data length: {}", mgr.len());
}

fn main() {
    run();
}