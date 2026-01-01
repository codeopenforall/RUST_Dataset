///////////////////////////////////////////////////////////////
// Rust code demonstrating unchecked error handling (CWE-252)
// This version ignores the result values from I/O operations.
// It uses unsafe pointer conversion and concurrency constructs.
///////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::OpenOptions;
use std::io::Write;

struct Worker {
    shared: Arc<Mutex<u32>>,
}

impl Worker {
    // Uses an unsafe block to simulate low-level pointer manipulation.
    // It attempts to update a resource by writing to a log file,
    // but errors from file open and write operations are ignored.
    unsafe fn update(&self, val: u32) {
        // Simulate unsafe pointer conversion
        let raw = Arc::as_ptr(&self.shared) as *const Mutex<u32>;
        let lock = &*raw;
        // Lock the mutex (assuming success)
        let _guard = lock.lock().unwrap();

        // Attempt to open a log file. On failure, the error is ignored.
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("data.log") {
            // Write to the file; ignore the Result!
            let _ = write!(file, "{}", val);
        }
    }

    // Runs several concurrent tasks that update the shared counter,
    // and also write to the same log file while ignoring error results.
    fn run(&self, iterations: u32) {
        let mut threads = Vec::new();
        for i in 0..iterations {
            let shared_clone = self.shared.clone();
            let handle = thread::spawn(move || {
                // Unsafe block for pointer conversion and direct access.
                unsafe {
                    let raw = Arc::as_ptr(&shared_clone) as *const Mutex<u32>;
                    let lock = &*raw;
                    let mut guard = lock.lock().unwrap();
                    *guard += i;
                }
                // Attempt to open a log file and write; errors are ignored.
                if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("data.log") {
                    let _ = write!(file, "{}", i);
                }
            });
            threads.push(handle);
        }
        // Join threads; any errors (panic) are ignored.
        for th in threads {
            let _ = th.join();
        }
    }
}

fn main() {
    // Initialize the shared resource.
    let counter = Arc::new(Mutex::new(0));
    let worker = Worker { shared: counter };

    // First update using unsafe code.
    unsafe {
        worker.update(100);
    }
    // Run additional updates concurrently.
    worker.run(10);
}