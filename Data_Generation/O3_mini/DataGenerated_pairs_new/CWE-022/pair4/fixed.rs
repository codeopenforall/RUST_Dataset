//////////////////////////////////////////////
// Corrected Code Example
//////////////////////////////////////////////
use std::path::{Path, PathBuf};
use std::fs::{File, canonicalize};
use std::io::{self, Read};
use std::sync::Arc;
use std::thread;

struct FileHandler {
    base_dir: PathBuf,
}

impl FileHandler {
    fn new(base: &str) -> Self {
        // Canonicalize the base directory at initialization to lock down the intended path.
        let base_dir = canonicalize(base).unwrap_or_else(|_| PathBuf::from(base));
        FileHandler { base_dir }
    }

    // Secure file reading function that validates the canonical path.
    fn read_file(&self, input: &str) -> io::Result<String> {
        let candidate = self.base_dir.join(input);
        // Resolve the candidate to its canonical form.
        let candidate_canon = canonicalize(&candidate)?;
        // Check that the resolved path is within the intended base directory.
        if !candidate_canon.starts_with(&self.base_dir) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Access outside the permitted directory is denied",
            ));
        }
        let mut file = File::open(&candidate_canon)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}

fn main() {
    // The base directory remains fixed.
    let handler = Arc::new(FileHandler::new("./data"));
    // Read input filename from command line arguments.
    let input = std::env::args().nth(1).unwrap_or_else(|| "default.txt".to_string());
    let handler_clone = Arc::clone(&handler);

    let worker = thread::spawn(move || {
        match handler_clone.read_file(&input) {
            Ok(text) => println!("File content:\n{}", text),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    });

    worker.join().unwrap();
}