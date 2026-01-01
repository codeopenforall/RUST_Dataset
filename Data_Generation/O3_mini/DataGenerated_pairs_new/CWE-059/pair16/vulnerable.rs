//////////////////////////////
// File: main.rs
//////////////////////////////
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

struct Manager {
    base: PathBuf,
}

impl Manager {
    fn new(dir: &str) -> Self {
        Manager {
            base: PathBuf::from(dir),
        }
    }

    // Processes a file given by a relative path.
    // It resolves the file path (including symbolic links) and then performs a file read
    // using an unsafe block and concurrent read simulation.
    fn process(&self, rel: &str) -> Result<String, String> {
        let candidate = self.base.join(rel);

        // Vulnerability: No check is done to ensure the canonical path is within the base directory.
        // An attacker-controlled symbolic link can resolve to a file outside the intended folder.
        let resolved = fs::canonicalize(&candidate).map_err(|e| e.to_string())?;

        // Unsafe block that creates a slice from the resolved path’s string.
        // This block is meant to simulate a low‐level processing that might be used in performance‐critical code.
        let path_str = resolved.to_str().ok_or("Invalid utf8 in path")?;
        let ptr = path_str.as_ptr();
        unsafe {
            // Create a slice from the raw pointer.
            // No actual validation is performed on the data referenced.
            let _slice = std::slice::from_raw_parts(ptr, path_str.len());
        }

        // Concurrent simulation: share the resolved path among threads
        let shared = Arc::new(Mutex::new(resolved.clone()));
        let shared_copy = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let locked = shared_copy.lock().unwrap();
            // Attempt to read file contents; if it points outside, this could be a security breach.
            fs::read_to_string(locked.to_str().unwrap()).unwrap_or_else(|_| String::new())
        });

        let thread_output = handle.join().map_err(|_| "Thread join failed".to_string())?;

        Ok(thread_output)
    }
}

fn main() {
    // For demonstration, we assume "safe_dir" is the intended directory.
    // However, a maliciously crafted symlink inside safe_dir (named "link.txt")
    // could point to a sensitive file outside.
    let mgr = Manager::new("safe_dir");
    match mgr.process("link.txt") {
        Ok(contents) => println!("Processed data: {}", contents),
        Err(e) => println!("Error encountered: {}", e),
    }
}