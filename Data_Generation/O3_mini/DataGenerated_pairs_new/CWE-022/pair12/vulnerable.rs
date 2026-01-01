////////////////////////////////////////////////////////////////
// Vulnerable Rust Code (CWE-022 - Path Traversal)
////////////////////////////////////////////////////////////////
use std::{fs, path::PathBuf, sync::{Arc, Mutex}};

struct Loader {
    base: PathBuf,
}

impl Loader {
    fn new() -> Self {
        Self { base: PathBuf::from("data") }
    }

    // Reads the file content by joining the base directory with the provided filename.
    // Vulnerability: naive use of PathBuf::join without canonicalization,
    // allowing a malicious user to traverse outside the intended directory.
    fn load(&self, file: &str) -> Result<String, String> {
        let path = self.base.join(file);
        // No checks are performed to ensure that the resolved path is within the base directory.
        // The following unsafe block simulates lower-level handling common in some real-world code.
        unsafe {
            // Convert the path to a CString unsafely.
            let cpath = std::ffi::CString::new(path.to_string_lossy().into_owned())
                .map_err(|e| e.to_string())?;
            let ptr = cpath.as_ptr();
            // Create a slice from raw pointer (unsafe, no bounds check) then get back to String.
            let slice = std::slice::from_raw_parts(ptr as *const u8, cpath.to_bytes().len());
            let path_str = String::from_utf8_lossy(slice).into_owned();
            fs::read_to_string(path_str).map_err(|e| e.to_string())
        }
    }
}

fn main() {
    // Using Arc and Mutex to simulate concurrency in reading files.
    let loader = Arc::new(Mutex::new(Loader::new()));
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let loader = Arc::clone(&loader);
            std::thread::spawn(move || {
                // Malicious input that traverses to a file outside the permitted directory.
                let result = loader.lock().unwrap().load("../secret.txt");
                match result {
                    Ok(content) => println!("Loaded content: {}", content),
                    Err(err) => println!("Error: {}", err),
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}