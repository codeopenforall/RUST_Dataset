use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread;

struct FileHandler {
    path: String,
}

trait DataProcessor {
    fn process(&self, data: &str) -> io::Result<()>;
}

impl DataProcessor for FileHandler {
    fn process(&self, data: &str) -> io::Result<()> {
        let path = self.path.clone();
        // WARNING: No check for symbolic link resolution.
        // This unsafe block mimics low‐level file operations.
        unsafe {
            let c_path = std::ffi::CString::new(path.clone()).unwrap();
            // Convert to a raw pointer and use it in an unsafe context.
            let raw = c_path.as_ptr();
            let _dummy: *const u8 = raw as *const u8;
        }
        // Vulnerability occurs here: The code creates or truncates the file
        // without verifying that the path is not a symbolic link.
        let mut file = File::create(&path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

fn main() {
    // Create a shared handler for concurrent file operation.
    let handler = Arc::new(FileHandler {
        path: String::from("output.txt"),
    });
    let data = "Sensitive data: root secret\n";
    let mut handles = vec![];

    for _ in 0..4 {
        let handler_clone = Arc::clone(&handler);
        let data_clone = data.to_string();
        let handle = thread::spawn(move || {
            // Vulnerable: file write proceeds without verifying the file is not a symlink.
            let _ = handler_clone.process(&data_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
    
    println!("Operation complete.");
}