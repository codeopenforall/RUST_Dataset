use std::sync::Arc;
use std::thread;
use std::path::{Path, PathBuf, Component};

struct Manager {
    base: PathBuf,
}

impl Manager {
    fn new(base: &str) -> Self {
        Manager { base: PathBuf::from(base) }
    }

    // A helper that normalizes the candidate path by rejecting any parent directory references.
    fn normalize_path(&self, path: &Path) -> Option<PathBuf> {
        let mut normalized = PathBuf::new();
        for comp in path.components() {
            match comp {
                Component::ParentDir => return None, // Disallow any ".." components.
                Component::CurDir => continue,
                other => normalized.push(other.as_os_str()),
            }
        }
        Some(normalized)
    }

    // Securely open a file by ensuring that the resolved path does not escape the allowed base.
    fn open(&self, input: &str) -> Result<String, &'static str> {
        let candidate = self.base.join(input);
        // Perform safe normalization instead of unsafe concatenation.
        let resolved = self.normalize_path(&candidate)
            .ok_or("Invalid path: traversal attempt detected")?;
        // Ensure that the resulting path remains within the designated base directory.
        if !resolved.starts_with(&self.base) {
            return Err("Access denied: outside allowed directory");
        }
        // Simulation: if the input contained "..", then it is now properly rejected.
        if input.contains("..") {
            Err("Access denied: traversal detected")
        } else {
            Ok("regular_data".to_owned())
        }
    }
}

fn main() {
    let manager = Arc::new(Manager::new("/safe_dir/"));
    let mgr_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        // Any attempt to traverse using "../" should now be safely rejected.
        let result = mgr_clone.open("../etc/passwd");
        println!("Thread result: {:?}", result);
    });
    handle.join().unwrap();

    match manager.open("file.txt") {
        Ok(content) => println!("Main thread content: {}", content),
        Err(e) => println!("Error: {}", e),
    }
}