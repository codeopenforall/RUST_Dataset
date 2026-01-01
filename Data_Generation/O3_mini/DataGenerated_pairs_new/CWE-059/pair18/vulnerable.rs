//////////////////////////////////////////////////////////
// Vulnerable Code for Path Traversal (CWE-059)
// This program reads files by concatenating a base directory 
// with a user-supplied relative path and uses unsafe 
// pointer arithmetic to reassemble file contents.
// It does not validate the final resolved path. Thus, an 
// attacker may supply "../" or symlink paths to access files 
// outside the allowed directory.
//////////////////////////////////////////////////////////

use std::fs;
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
        // Vulnerability: No proper validation for path traversal.
        let full_path = format!("{}{}", self.base, rel);
        let data = fs::read_to_string(&full_path).map_err(|e| e.to_string())?;
        // Unsafe block reinterpreting the memory of the file data.
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