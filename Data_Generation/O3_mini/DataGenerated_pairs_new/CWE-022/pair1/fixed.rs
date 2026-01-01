//////////////////////////////
// Fixed Code Example
//////////////////////////////

use std::fs::File;
use std::io::{Read, Error as IoError, ErrorKind};
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

    // This method securely resolves the file path by canonicalizing
    // and ensuring the resulting path is within the designated root.
    fn fetch(&self, user_input: &str) -> Result<String, IoError> {
        let candidate = self.root.join(user_input);
        // Canonicalize both the candidate and the base directory.
        let candidate_canon = candidate.canonicalize()?;
        let base_canon = self.root.canonicalize()?;
        // Reject if candidate path escapes the defined root directory.
        if !candidate_canon.starts_with(&base_canon) {
            return Err(IoError::new(
                ErrorKind::PermissionDenied,
                "access denied: outside permitted directory",
            ));
        }
        // The unsafe block is retained to mimic legacy low-level operations,
        // but now it's used on a validated file path.
        unsafe {
            let mut file = File::open(candidate_canon)?;
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
                println!("Thread read (unexpected success): {}", &data[0..std::cmp::min(20, data.len())]);
            }
            Err(err) => {
                println!("Thread correctly failed: {}", err);
            }
        }
    });

    // Main thread performing the secure operation.
    let res = handler.lock().unwrap().fetch("../Cargo.toml");
    match res {
        Ok(data) => {
            println!("Main thread read (unexpected success): {}", &data[0..std::cmp::min(20, data.len())]);
        }
        Err(err) => {
            println!("Main thread correctly failed: {}", err);
        }
    }

    handle.join().unwrap();
}