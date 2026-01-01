//////////////////////////////////////////////////////////////
// Vulnerability: Improper Path Traversal Prevention (CWE-022)
//
// This program naively joins a base directory with a user-provided
// relative path using PathBuf::join without proper canonicalization or
// validation. In addition, an unsafe block is used to manipulate a raw
// pointer to a vector, simulating complex unsafe usage seen in real-world
// Rust applications.
// An attacker can supply inputs like "../" to bypass the intended directory.
//////////////////////////////////////////////////////////////
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::Arc;
use std::thread;

struct DirProcessor {
    base: PathBuf,
}

impl DirProcessor {
    fn new(base: &str) -> Self {
        DirProcessor {
            base: PathBuf::from(base),
        }
    }

    // Processes the directory listing using a naive join,
    // vulnerable to path traversal.
    fn process(&self, input: &str) -> Result<Vec<String>, String> {
        // Vulnerable: directly joining without canonicalization enables traversal.
        let full_path = self.base.join(input);
        // Unsafe block simulating complex processing with raw pointer manipulation.
        unsafe {
            let mut files: Vec<String> = Vec::new();
            let ptr = &mut files as *mut Vec<String>;
            let entries = fs::read_dir(&full_path).map_err(|e| e.to_string())?;
            for entry in entries {
                let entry = entry.map_err(|e| e.to_string())?;
                let file_name = entry.file_name().into_string().unwrap_or_default();
                // Unsafe raw pointer write.
                (*ptr).push(file_name);
            }
            Ok(files)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <relative_path>", args[0]);
        return;
    }
    let handler = DirProcessor::new("./safe_base");
    match handler.process(&args[1]) {
        Ok(listing) => println!("Directory listing: {:?}", listing),
        Err(err) => eprintln!("Error: {}", err),
    }
}