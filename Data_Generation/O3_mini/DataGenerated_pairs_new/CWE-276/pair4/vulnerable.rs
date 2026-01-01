//////////////////////////////
// Insecure implementation //
//////////////////////////////

use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

struct Handler;

impl Handler {
    fn new() -> Self {
        Handler
    }

    fn create_and_write(&self, path: &str, data: &str) -> std::io::Result<()> {
        // Create file with overly permissive permissions (0o777).
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o777)  // Insecure: overly permissive mode.
            .open(path)?;
        let mut file = file;

        // Unsafe block simulating pointer manipulation vulnerability.
        unsafe {
            let raw_ptr = &mut file as *mut File;
            // Performing pointless pointer arithmetic that mimics risky practices.
            let _alias = raw_ptr.offset(0);
        }

        // Concurrent writes using a shared smart pointer.
        let shared_file = Arc::new(Mutex::new(file));
        let mut handles = vec![];

        for _ in 0..3 {
            let file_clone = Arc::clone(&shared_file);
            let data_clone = data.to_owned();
            let handle = thread::spawn(move || -> std::io::Result<()> {
                let mut guard = file_clone.lock().unwrap();
                guard.write_all(data_clone.as_bytes())?;
                Ok(())
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap()?;
        }
        Ok(())
    }
}

fn main() {
    let manager = Handler::new();
    let file_path = "example.txt";
    let info = "Sensitive data";
    if let Err(e) = manager.create_and_write(file_path, info) {
        eprintln!("Error encountered: {}", e);
        std::process::exit(1);
    }
    println!("Operation completed (insecure implementation).");
}