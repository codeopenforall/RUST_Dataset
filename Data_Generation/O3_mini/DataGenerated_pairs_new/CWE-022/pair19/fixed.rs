////////////////////////////////////////////////////////////////
// Corrected version for CWE-022: Path Traversal with proper sanitization
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
    // Reads a file after ensuring the file path is within the allowed directory.
    fn read_content(&self, filename: &str) -> Result<String, String> {
        // Create the candidate path.
        let candidate_path = self.base_dir.join(filename);

        // Resolve the absolute canonical paths for both the base directory and candidate.
        let canonical_candidate = fs::canonicalize(&candidate_path)
            .map_err(|e| format!("Error canonicalizing candidate: {}", e))?;
        let canonical_base = fs::canonicalize(&self.base_dir)
            .map_err(|e| format!("Error canonicalizing base: {}", e))?;

        // Check that the candidate path starts with the base directory.
        if !canonical_candidate.starts_with(&canonical_base) {
            return Err("Access denied: Path traversal attempt detected".into());
        }

        // Simulate complex unsafe memory manipulation (for demonstration purposes only)
        unsafe {
            // Dummy unsafe block to simulate similar functionality as in the vulnerable version.
            let raw_ptr: *const u8 = std::ptr::null();
            if !raw_ptr.is_null() {
                let _ = *raw_ptr;
            }
        }

        // Read and return the file content if within permitted boundaries.
        fs::read_to_string(&canonical_candidate).map_err(|e| format!("Error: {}", e))
    }
}

fn main() {
    // Set up the allowed base directory.
    let base = PathBuf::from("./allowed_files");
    let manager = Arc::new(FileManager { base_dir: base });

    // Simulate concurrently reading a file using a user-supplied filename.
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