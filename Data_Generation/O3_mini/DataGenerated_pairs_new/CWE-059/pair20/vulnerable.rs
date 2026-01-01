////////////////////////////////////////////////////////////
// Vulnerable Rust Code
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

    // Checks if the provided path is a symlink.
    fn is_link(&self) -> bool {
        if let Ok(metadata) = fs::symlink_metadata(&self.path) {
            metadata.file_type().is_symlink()
        } else {
            false
        }
    }

    // Resolves the path, but unsafely uses raw pointer conversion and subsequently does not
    // validate that the resolved target is within an allowed directory.
    fn resolve(&self) -> PathBuf {
        if self.is_link() {
            unsafe {
                let raw_ptr = self.path.as_ptr();                 // Vulnerability starts here.
                let length = self.path.len();
                let unsafe_str = std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, length));
                if let Ok(target) = fs::read_link(unsafe_str) {
                    return target;                                // Returns an unchecked symlink target.
                }
            }
        }
        PathBuf::from(&self.path)
    }
}

fn process_file(input: &str) -> io::Result<String> {
    let resolver = PathResolver::new(input);
    let resolved_path = resolver.resolve();
    
    // Opens the file at the resolved path without verifying if it is inside an allowed area.
    let mut file = File::open(&resolved_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    // Simulate a race-condition by concurrently writing to the file.
    let shared_path = Arc::new(resolved_path);
    let sp_clone = Arc::clone(&shared_path);
    let handle = thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(10));
        if let Ok(mut f) = OpenOptions::new().write(true).open(&*sp_clone) {
            let _ = f.write_all(b"malicious update");
        }
    });
    handle.join().unwrap();
    Ok(content)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Default to "test.txt" if no argument is provided.
    let file_path = if args.len() > 1 { &args[1] } else { "test.txt" };
    match process_file(file_path) {
        Ok(text) => println!("File content: {}", text),
        Err(e) => eprintln!("Error encountered: {}", e),
    }
}