///////////////////////////////////////////////////////////////////////////////
// This code fixes the symbolic link resolution vulnerability by leveraging
// Rust’s safe standard library functions. The file path is canonicalized
// and then validated to ensure it remains within a designated safe directory.
// Unsafe raw pointer manipulations are completely removed.
///////////////////////////////////////////////////////////////////////////////

use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Resolver;

impl Resolver {
    // Secure path resolution: Canonicalizes the path and ensures it is within the
    // allowed "safe" directory.
    pub fn resolve(&self, input: &str) -> Result<String, String> {
        let path = Path::new(input);
        let canonical = fs::canonicalize(path)
            .map_err(|e| format!("Canonicalization error: {}", e))?;
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        let safe_dir = current_dir.join("safe");
        if !canonical.starts_with(&safe_dir) {
            return Err("Access denied: path is outside the safe directory".to_string());
        }
        canonical
            .into_os_string()
            .into_string()
            .map_err(|_| "Failed to convert path".to_string())
    }
}

fn process(input: &str) -> Result<String, String> {
    let resolver = Resolver;
    let resolved_path = resolver.resolve(input)?;

    let file = File::open(&resolved_path)
        .map_err(|e| format!("Error opening file: {}", e))?;
    let mut buf_reader = std::io::BufReader::new(file);
    let mut content = String::new();
    buf_reader
        .read_to_string(&mut content)
        .map_err(|e| format!("Error reading file: {}", e))?;
    Ok(content)
}

fn main() {
    // Ensure the safe directory exists.
    let safe_dir = std::env::current_dir().unwrap().join("safe");
    fs::create_dir_all(&safe_dir).unwrap();

    // Shared mutable state via a Mutex and Arc; a worker thread simulates
    // concurrent file access.
    let shared_result = Arc::new(Mutex::new(String::new()));
    let shared_result_clone = Arc::clone(&shared_result);

    // Accept input path from command-line arguments.
    let args: Vec<String> = std::env::args().collect();
    let input_path = if args.len() > 1 {
        args[1].clone()
    } else {
        // Default to a file in the safe directory.
        safe_dir.join("test_file.txt").into_os_string().into_string().unwrap()
    };

    let handle = thread::spawn(move || {
        match process(&input_path) {
            Ok(content) => {
                let mut data = shared_result_clone.lock().unwrap();
                *data = content;
            }
            Err(e) => {
                panic!("Operation failed: {}", e);
            }
        }
    });

    handle.join().unwrap();
    let result = shared_result.lock().unwrap();
    println!("File content: {}", *result);
}