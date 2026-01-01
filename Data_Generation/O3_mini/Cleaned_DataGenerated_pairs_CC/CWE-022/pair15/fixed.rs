use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
struct FileManager {
    root: PathBuf,
}
impl FileManager {
    fn new<P: AsRef<Path>>(root: P) -> Self {
        FileManager {
            root: root.as_ref().to_path_buf(),
        }
    }
    fn read_file(&self, file: &str) -> io::Result<String> {
        let candidate = self.root.join(file).canonicalize()?;
        let canonical_root = self.root.canonicalize()?;
        if !candidate.starts_with(&canonical_root) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Path traversal detected"));
        }
        let bytes = fs::read(&candidate)?;
        let text = unsafe { std::str::from_utf8_unchecked(&bytes) }.to_string();
        Ok(text)
    }
}
fn main() {
    let base_dir = env::current_dir().unwrap().join("allowed");
    let _ = fs::create_dir_all(&base_dir);
    let manager = Arc::new(FileManager::new(&base_dir));
    let mut handles = vec![];
    for _ in 0..3 {
        let mgr = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            match mgr.read_file("data.txt") {
                Ok(content) => println!("File read: {}", content),
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        });
        handles.push(handle);
    }
    for h in handles {
        let _ = h.join();
    }
    match manager.read_file("../Cargo.toml") {
        Ok(content) => println!("Unexpected access: Read content: {}...", &content[0..30.min(content.len())]),
        Err(e) => println!("Access blocked: {}", e),
    }
}
