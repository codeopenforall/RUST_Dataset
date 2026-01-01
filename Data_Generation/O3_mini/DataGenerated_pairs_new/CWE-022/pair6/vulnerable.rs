/* Vulnerable version */
use std::fs;
use std::path::{PathBuf, Path};
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CString, CStr};

pub struct FileManager {
    base: PathBuf,
}

impl FileManager {
    pub fn new(base: PathBuf) -> Self {
        FileManager { base }
    }

    // This method naively appends the user-provided path to the base directory.
    // It uses unsafe blocks to manipulate CStrings and then passes the pointer into
    // a file access function without any canonicalization or validation.
    pub fn open_file(&self, user_input: &str) -> Result<String, String> {
        // Vulnerability: Using naive join that does not prevent traversal (../ etc)
        let path = self.base.join(user_input);
        // Unsafe conversion of the Path to a C-string for simulating a low-level API call.
        unsafe {
            let path_str = path.to_str().ok_or_else(|| "Invalid path".to_string())?;
            let c_path = CString::new(path_str).map_err(|_| "Failed to convert path")?;
            let raw_ptr = c_path.as_ptr();
            // Simulate improper usage of unsafe by reinterpreting the pointer directly.
            let file_path = CStr::from_ptr(raw_ptr)
                .to_str()
                .map_err(|_| "Conversion error")?;
            let content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
            Ok(content)
        }
    }
}

fn main() {
    // Base directory intentionally set
    let base_dir = PathBuf::from("./data");
    let manager = Arc::new(Mutex::new(FileManager::new(base_dir)));

    // For demonstration, we spawn a thread to simulate concurrent access.
    let handles: Vec<_> = (0..1).map(|_| {
        let mgr = Arc::clone(&manager);
        thread::spawn(move || {
            let fm = mgr.lock().unwrap();
            // In production, the input could be provided by the user.
            // Here we use a benign filename so main runs without error.
            match fm.open_file("test.txt") {
                Ok(content) => println!("Content: {}", content),
                Err(e) => eprintln!("Error: {}", e),
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}