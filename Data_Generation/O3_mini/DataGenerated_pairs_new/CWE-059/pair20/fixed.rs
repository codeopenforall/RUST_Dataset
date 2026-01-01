////////////////////////////////////////////////////////////
// Corrected Rust Code
////////////////////////////////////////////////////////////
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

struct PathResolver {
    path: String,
}

impl PathResolver {
    fn new(input: &str) -> Self {
        Self { path: input.to_owned() }
    }

    // Resolves the provided path safely by canonicalizing it.
    fn resolve(&self) -> io::Result<PathBuf> {
        let raw_path = PathBuf::from(&self.path);
        // Canonicalize to resolve symlinks and remove ".." components.
        let abs_path = fs::canonicalize(raw_path)?;
        // Define an allowed base directory.
        let allowed = fs::canonicalize("./allowed")?;
        // Ensure the canonical (resolved) path is within the allowed directory.
        if !abs_path.starts_with(&allowed) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access not permitted"));
        }
        Ok(abs_path)
    }
}

fn process_file(input: &str) -> io::Result<String> {
    let resolver = PathResolver::new(input);
    let resolved_path = resolver.resolve()?;
    
    // Open the file after confirming it is within an allowed directory.
    let mut file = File::open(&resolved_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    // Simulate concurrent update, but now the file access is atomic and happens within the allowed space.
    let shared_path = Arc::new(resolved_path);
    let sp_clone = Arc::clone(&shared_path);
    let handle = thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(10));
        // Use safe options for writing.
        if let Ok(mut f) = OpenOptions::new().write(true).create(false).open(&*sp_clone) {
            let _ = f.write_all(b"safe update");
        }
    });
    handle.join().unwrap();
    Ok(content)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Default to "./allowed/test.txt" if no argument is provided.
    let file_path = if args.len() > 1 { &args[1] } else { "./allowed/test.txt" };
    match process_file(file_path) {
        Ok(text) => println!("File content: {}", text),
        Err(e) => eprintln!("Error encountered: {}", e),
    }
}