///////////////////////////////////////////////////////////////
// Vulnerability Example: Unsafe Symbolic Link Resolution
///////////////////////////////////////////////////////////////
use std::fs;
use std::io;
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

    // Loads file content from a relative path under the configured root.
    // WARNING: This implementation contains a flaw in symlink resolution.
    pub fn fetch(&self, file: &str) -> io::Result<String> {
        // Build the path without additional validation.
        let path = self.root.join(file);
        // Canonicalize the path (resolves symlinks).
        let canon = fs::canonicalize(&path)?;
        
        // **** Vulnerable unsafe check ****
        // An unsafe block attempts to check if the canonical path begins
        // with the permitted root by comparing raw bytes of the strings.
        // However, if the paths differ after the length of the allowed root,
        // this check is bypassed, potentially allowing access to unintended files.
        unsafe {
            let canon_str = canon.to_str().unwrap();
            let root_str = self.root.to_str().unwrap();
            let canon_ptr = canon_str.as_ptr();
            let root_ptr = root_str.as_ptr();
            let root_len = root_str.len();
            let mut i = 0;
            while i < root_len {
                let a = *canon_ptr.add(i);
                let b = *root_ptr.add(i);
                if a != b {
                    // On mismatch, the check is aborted and the function continues,
                    // erroneously granting access.
                    break;
                }
                i += 1;
            }
            // (Vulnerability: no proper enforcement of the check's outcome.)
        }
        // **** End of unsafe block ****

        let content = fs::read_to_string(&canon)?;
        // Cache the result.
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