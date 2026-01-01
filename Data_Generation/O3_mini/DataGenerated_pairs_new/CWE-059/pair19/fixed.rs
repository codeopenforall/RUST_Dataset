///////////////////////////////////////////////////////////////
// Fixed Example: Proper Symbolic Link Resolution Validation
///////////////////////////////////////////////////////////////
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

struct Loader {
    root: PathBuf,
    cache: Arc<Mutex<Vec<(PathBuf, String)>>>,
}

impl Loader {
    pub fn new(base: &str) -> Self {
        Self {
            root: PathBuf::from(base),
            cache: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Securely loads file content ensuring the resolved path
    // lies within the allowed base directory.
    pub fn fetch(&self, file: &str) -> io::Result<String> {
        let path = self.root.join(file);
        let canon = fs::canonicalize(&path)?;
        
        // Fixed check: using PathBuf's starts_with method to ensure
        // that the canonical path is indeed within the allowed root.
        if !canon.starts_with(&self.root) {
            return Err(io::Error::new(
                ErrorKind::PermissionDenied,
                "Access denied: resolved path lies outside the allowed directory",
            ));
        }
        
        let content = fs::read_to_string(&canon)?;
        let mut cache_lock = self.cache.lock().unwrap();
        cache_lock.push((canon, content.clone()));
        Ok(content)
    }
}

fn main() {
    // Assume a designated safe directory "safe_dir" exists.
    let loader = Loader::new("safe_dir");
    // For demonstration, attempt to load "test.txt" from "safe_dir".
    match loader.fetch("test.txt") {
        Ok(data) => println!("Loaded content:\n{}", data),
        Err(e) => eprintln!("Error encountered: {}", e),
    }
}