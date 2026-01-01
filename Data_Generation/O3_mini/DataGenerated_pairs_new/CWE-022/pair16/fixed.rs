//////////////////////////////////////////////////////////////
// Fix: Proper Path Traversal Prevention (CWE-022)
// 
// The updated version validates the user-supplied path by canonicalizing
// both the base directory and the joined path. It then ensures that the resulting
// path begins with the expected base directory. This prevents path traversal
// attacks. The unsafe block has been replaced with safe Rust code.
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
        // Canonicalize the base directory at creation.
        let canonical_base = fs::canonicalize(base).unwrap_or_else(|_| PathBuf::from(base));
        DirProcessor {
            base: canonical_base,
        }
    }

    // Processes the directory listing after validating the full path.
    fn process(&self, input: &str) -> Result<Vec<String>, String> {
        // Join the base directory with the provided input.
        let joined = self.base.join(input);
        // Canonicalize the joined path.
        let canonical_path = fs::canonicalize(&joined).map_err(|e| e.to_string())?;
        // Verify that the canonical path starts with the base directory.
        if !canonical_path.starts_with(&self.base) {
            return Err("Invalid path: Access outside base directory denied".to_string());
        }
        let mut files: Vec<String> = Vec::new();
        // Safely iterate over directory entries.
        let entries = fs::read_dir(&canonical_path).map_err(|e| e.to_string())?;
        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_name = entry.file_name().into_string().unwrap_or_default();
            files.push(file_name);
        }
        Ok(files)
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