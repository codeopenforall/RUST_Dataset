///////////////////////////////////////////////////////////////
// Rust code with proper error handling for I/O operations.
// This version checks all I/O related Result values and propagates errors.
// It still uses unsafe blocks and concurrency, but no errors are silently ignored.
///////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::OpenOptions;
use std::io::{Write, Result, Error, ErrorKind};

struct Worker {
    shared: Arc<Mutex<u32>>,
}

impl Worker {
    // The unsafe block remains for low-level pointer manipulation.
    // Now it returns a Result, propagating any errors from file operations.
    unsafe fn update(&self, val: u32) -> Result<()> {
        let raw = Arc::as_ptr(&self.shared) as *const Mutex<u32>;
        let lock = &*raw;
        // Properly check for poisoning errors.
        let _guard = lock.lock().map_err(|_| Error::new(ErrorKind::Other, "mutex poisoned"))?;
        
        // Open the log file and propagate any errors.
        let mut file = OpenOptions::new().create(true).append(true).open("data.log")?;
        write!(file, "{}", val)?;
        Ok(())
    }

    // Concurrently runs tasks and propagates any I/O errors from those tasks.
    fn run(&self, iterations: u32) -> Result<()> {
        let mut threads = Vec::new();
        for i in 0..iterations {
            let shared_clone = self.shared.clone();
            let handle = thread::spawn(move || -> Result<()> {
                unsafe {
                    let raw = Arc::as_ptr(&shared_clone) as *const Mutex<u32>;
                    let lock = &*raw;
                    let mut guard = lock.lock().map_err(|_| Error::new(ErrorKind::Other, "mutex poisoned"))?;
                    *guard += i;
                }
                let mut file = OpenOptions::new().create(true).append(true).open("data.log")?;
                write!(file, "{}", i)?;
                Ok(())
            });
            threads.push(handle);
        }
        for th in threads {
            // Propagate errors from the spawned threads.
            th.join().map_err(|_| Error::new(ErrorKind::Other, "thread panicked"))??;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let counter = Arc::new(Mutex::new(0));
    let worker = Worker { shared: counter };

    unsafe {
        // Propagate errors if the update fails.
        worker.update(100)?;
    }
    worker.run(10)?;
    Ok(())
}