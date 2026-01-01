//////////////////////////////////////////////////////////
// Fixed Code for Path Traversal Protection (CWE-059)
// This corrected program canonicalizes the resulting file path 
// and verifies that it lies within the intended base directory. 
// If validation fails, the file access is rejected.
//////////////////////////////////////////////////////////

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

struct Loader {
    base: String,
}

trait FileAccess {
    fn get_content(&self, rel: &str) -> Result<String, String>;
}

impl FileAccess for Loader {
    fn get_content(&self, rel: &str) -> Result<String, String> {
        // Build the candidate path.
        let candidate = Path::new(&self.base).join(rel);
        
        // Canonicalize both the candidate and allowed directory.
        let canon_candidate = candidate.canonicalize().map_err(|e| e.to_string())?;
        let canon_base = Path::new(&self.base).canonicalize().map_err(|e| e.to_string())?;
        
        // Validate that the candidate path starts with the allowed base directory.
        if !canon_candidate.starts_with(&canon_base) {
            return Err("Access denied: path traversal attempt detected".to_string());
        }
        
        let data = fs::read_to_string(&candidate).map_err(|e| e.to_string())?;
        // Retain the unsafe block for processing file data.
        unsafe {
            let ptr = data.as_ptr();
            let len = data.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            Ok(String::from_utf8_lossy(slice).into_owned())
        }
    }
}

fn main() {
    let loader = Loader { base: "allowed/".to_string() };
    let shared_results = Arc::new(Mutex::new(Vec::new()));
    let loader_arc = Arc::new(loader);

    // Spawn multiple threads to simulate concurrent file access.
    let handles: Vec<_> = (0..3)
        .map(|i| {
            let loader_thread = Arc::clone(&loader_arc);
            let results_thread = Arc::clone(&shared_results);
            thread::spawn(move || {
                // For demonstration, using file names "file0.txt", "file1.txt", etc.
                let path = format!("file{}.txt", i);
                let res = loader_thread.get_content(&path);
                results_thread.lock().unwrap().push(res);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let results = shared_results.lock().unwrap();
    for entry in results.iter() {
        match entry {
            Ok(content) => println!("File content: {}", content),
            Err(err) => println!("Error: {}", err),
        }
    }
}