//////////////////////////////
// Vulnerable Code Example
//////////////////////////////

use std::fs::File;
use std::io::{Read, Error as IoError};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

struct FileHandler {
    root: PathBuf,
}

impl FileHandler {
    fn new(root: PathBuf) -> Self {
        FileHandler { root }
    }

    // This method naively joins the user input with the root path.
    // It does not check for path traversal, allowing inputs like "../" to escape the intended directory.
    fn fetch(&self, user_input: &str) -> Result<String, IoError> {
        // Vulnerable join without checking for canonicalization or parent traversal.
        let joined = self.root.join(user_input);
        // Unsafe block simulating low-level operations that might bypass typical Rust safety checks.
        unsafe {
            let mut file = File::open(joined)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(content)
        }
    }
}

fn main() {
    // Base directory is intended to restrict file accesses.
    let base_dir = PathBuf::from("./safe_dir");
    let handler = Arc::new(Mutex::new(FileHandler::new(base_dir)));

    // Simulate concurrent access.
    let handler_clone = Arc::clone(&handler);
    let handle = thread::spawn(move || {
        // Malicious input attempting path traversal.
        let result = handler_clone.lock().unwrap().fetch("../Cargo.toml");
        match result {
            Ok(data) => {
                // For demonstration, show first 20 characters.
                println!("Thread completed read: {}", &data[0..std::cmp::min(20, data.len())]);
            }
            Err(err) => {
                eprintln!("Thread got error: {}", err);
            }
        }
    });

    // Main thread performing the same vulnerable operation.
    let res = handler.lock().unwrap().fetch("../Cargo.toml");
    match res {
        Ok(data) => {
            println!("Main thread read: {}", &data[0..std::cmp::min(20, data.len())]);
        }
        Err(err) => {
            eprintln!("Main thread error: {}", err);
        }
    }

    handle.join().unwrap();
}