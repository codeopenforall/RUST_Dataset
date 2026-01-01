////////////////////////////////////////////////////////////////
// Vulnerable version for CWE-022: Path Traversal using unsafe code
////////////////////////////////////////////////////////////////
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::sync::Arc;

// A structure that manages file access within a base directory.
struct FileManager {
    base_dir: PathBuf,
}

impl FileManager {
    // Reads a file whose path is relative to the base_dir.
    // This implementation is vulnerable to path traversal attacks.
    fn read_content(&self, filename: &str) -> Result<String, String> {
        // Vulnerability: using join without proper sanitization or canonicalization
        let target_path = self.base_dir.join(filename); // Vulnerability occurs here

        // Simulate complex unsafe memory manipulation (for demonstration purposes only)
        unsafe {
            // Create a raw pointer dummy access
            let raw_ptr: *const u8 = std::ptr::null();
            // Fake unsafe usage, e.g., simulating concurrent low-level operations.
            if !raw_ptr.is_null() {
                // This block is never reached but mimics unsafe practice.
                let _ = *raw_ptr;
            }
        }

        // Attempt to read the file content
        fs::read_to_string(&target_path).map_err(|e| format!("Error: {}", e))
    }
}

fn main() {
    // Set up a base directory expected to contain allowed files.
    let base = PathBuf::from("./allowed_files");
    let manager = Arc::new(FileManager { base_dir: base });

    // Simulate concurrently reading a file using a user-supplied filename.
    // If an attacker supplies "../secret.txt", they can traverse outside the intended directory.
    let filenames = vec!["test.txt", "../secret.txt"];
    let mut handles = vec![];

    for name in filenames {
        let mgr = Arc::clone(&manager);
        // Spawn threads to simulate concurrent file accesses.
        let handle = thread::spawn(move || {
            match mgr.read_content(name) {
                Ok(content) => content,
                Err(err) => err,
            }
        });
        handles.push(handle);
    }

    // Join all thread handles and print the output.
    for handle in handles {
        match handle.join() {
            Ok(result) => println!("Output: {}", result),
            Err(_) => println!("Thread panicked"),
        }
    }
}