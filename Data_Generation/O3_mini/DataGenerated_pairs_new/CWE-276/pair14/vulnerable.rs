#![allow(dead_code)]
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::Write;

struct FileHandler {
    filepath: String,
}

impl FileHandler {
    // This function creates a file with insecure permissions.
    unsafe fn create(&self) -> std::io::Result<File> {
        // Vulnerable: using overly permissive file mode 0o777 (CWE-276)
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o777)     // Vulnerability present here (line 19)
            .open(&self.filepath)?;
        Ok(file)
    }

    unsafe fn process(&self) -> std::io::Result<()> {
        let mut file = self.create()?;
        // Example unsafe operation to simulate complex unsafe behavior.
        let buf: *mut u8 = &mut 42u8 as *mut u8;
        unsafe {
            *buf = 100; // arbitrary unsafe operation
        }
        file.write_all(b"data")?;
        Ok(())
    }
}

fn main() {
    let handler = Arc::new(Mutex::new(FileHandler { filepath: "output.txt".to_string() }));
    let mut handles = vec![];

    // Spawn multiple threads to simulate concurrent file creation.
    for _ in 0..4 {
        let h = Arc::clone(&handler);
        let th = thread::spawn(move || {
            let handler_lock = h.lock().unwrap();
            unsafe {
                handler_lock.process().unwrap();
            }
        });
        handles.push(th);
    }
    for th in handles {
        th.join().unwrap();
    }
    println!("Operation completed in insecure mode.");
}