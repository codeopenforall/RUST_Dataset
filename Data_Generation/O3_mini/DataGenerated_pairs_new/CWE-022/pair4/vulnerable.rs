//////////////////////////////////////////////
// Vulnerable Code Example
//////////////////////////////////////////////
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, Read};
use std::sync::Arc;
use std::thread;

struct FileHandler {
    base_dir: PathBuf,
}

impl FileHandler {
    fn new(base: &str) -> Self {
        FileHandler {
            base_dir: PathBuf::from(base),
        }
    }

    // This routine reads a file by naively concatenating the base_dir with the provided input.
    // It does not canonicalize or validate the resulting path.
    fn read_file(&self, input: &str) -> io::Result<String> {
        // Vulnerability: Using a simple join leads to path traversal if `input` contains "../" constructs.
        let target_path = self.base_dir.join(input);
        // Simulated unsafe block: using an unsafe conversion to mimic low-level operations.
        let mut file = unsafe {
            // This unsafe block wraps the file open operation (for demonstration purposes only).
            File::open(&target_path)?
        };
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}

fn main() {
    // The base directory intended for file accesses.
    let handler = Arc::new(FileHandler::new("./data"));
    // Read input filename from command line arguments; could be malicious (e.g. "../secret.txt").
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