////////////////// Corrected Version //////////////////
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

pub struct Loader {
    base_dir: PathBuf,
}

impl Loader {
    pub fn new(base: &str) -> Self {
        Loader { base_dir: PathBuf::from(base) }
    }

    // Safely loads a file only if its canonicalized path is within the base directory.
    pub fn load_file(&self, name: &str) -> Result<String, String> {
        // Combine base directory with user input.
        let joined = self.base_dir.join(name);

        // Canonicalize the joined path and the base directory.
        let canonical = joined
            .canonicalize()
            .map_err(|e| format!("Canonicalization error: {}", e))?;
        let base_canonical = self.base_dir
            .canonicalize()
            .map_err(|e| format!("Base canonicalization error: {}", e))?;

        // FIX: Verify that the resulting path is within the base directory.
        if !canonical.starts_with(&base_canonical) {
            return Err("Access denied: path traversal detected!".to_string());
        }

        let mut file = File::open(&canonical)
            .map_err(|e| format!("File open error: {}", e))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Read error: {}", e))?;
        // Unsafe conversion remains for demonstration; assume input is valid UTF-8.
        unsafe {
            let s = std::str::from_utf8_unchecked(&buffer);
            Ok(s.to_string())
        }
    }
}

fn main() {
    let loader = Arc::new(Loader::new("./data"));
    let loader_clone = Arc::clone(&loader);
    let handle = thread::spawn(move || {
        match loader_clone.load_file("example.txt") {
            Ok(content) => println!("Thread file content: {}", content),
            Err(e) => println!("Thread error: {}", e),
        }
    });
    handle.join().unwrap();
    println!("Main thread finished execution.");
}