//// Fixed code snippet for CWE-022 (Path Traversal) in Rust
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

struct FileManager {
    root: PathBuf,
}

impl FileManager {
    fn new<P: AsRef<Path>>(root: P) -> Self {
        FileManager {
            root: root.as_ref().to_path_buf(),
        }
    }

    // Corrected function with canonicalization and prefix checking to avoid path traversal.
    fn read_file(&self, file: &str) -> io::Result<String> {
        let candidate = self.root.join(file).canonicalize()?;
        let canonical_root = self.root.canonicalize()?;
        // Enforce that the resolved candidate path is within the canonical root directory.
        if !candidate.starts_with(&canonical_root) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Path traversal detected"));
        }
        let bytes = fs::read(&candidate)?;
        // Although unsafe block remains for demonstration, the conversion now happens on controlled input.
        let text = unsafe { std::str::from_utf8_unchecked(&bytes) }.to_string();
        Ok(text)
    }
}

fn main() {
    // Simulate intended directory in a controlled environment.
    let base_dir = env::current_dir().unwrap().join("allowed");
    // Create the allowed directory if it doesn't exist.
    let _ = fs::create_dir_all(&base_dir);

    let manager = Arc::new(FileManager::new(&base_dir));
    let mut handles = vec![];

    // Spawn a few threads to simulate concurrent file access.
    for _ in 0..3 {
        let mgr = Arc::clone(&manager);
        // Using a benign file name that is assumed to be in allowed/.
        let handle = thread::spawn(move || {
            match mgr.read_file("data.txt") {
                Ok(content) => println!("File read: {}", content),
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        });
        handles.push(handle);
    }

    // Wait for threads to complete.
    for h in handles {
        let _ = h.join();
    }
    
    // Attempt to read with a path traversal string; this should be rejected.
    match manager.read_file("../Cargo.toml") {
        Ok(content) => println!("Unexpected access: Read content: {}...", &content[0..30.min(content.len())]),
        Err(e) => println!("Access blocked: {}", e),
    }
}