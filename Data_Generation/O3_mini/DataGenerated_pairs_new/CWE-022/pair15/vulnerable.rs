//// Vulnerable code snippet for CWE-022 (Path Traversal) in Rust
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

    // Vulnerable function which joins user-provided input without canonicalization.
    fn read_file(&self, file: &str) -> io::Result<String> {
        // NOTE: The join below is vulnerable to path traversal because it does not
        // verify that the resulting path remains under the intended base directory.
        let path = self.root.join(file); // <-- Vulnerability: CWE-022 (Path Traversal)
        let bytes = fs::read(&path)?;
        // Unsafe conversion from bytes to str (bypasses utf8 check), simulating use of unsafe blocks.
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
        // Using a benign file name that is presumed to exist in allowed/.
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
    
    // Main thread demonstrating a potential exploit:
    // A malicious user can supply a path traversal string to escape the allowed directory.
    match manager.read_file("../Cargo.toml") {
        Ok(content) => println!("Exploit successful! Read content: {}...", &content[0..30.min(content.len())]),
        Err(e) => println!("Exploit prevented: {}", e),
    }
}