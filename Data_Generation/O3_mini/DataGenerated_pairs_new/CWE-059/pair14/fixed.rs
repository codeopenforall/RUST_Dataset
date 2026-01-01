/* 
   This Rust code corrects the file access operation by enforcing a security check on the 
   resolved canonical path. After canonicalization, it ensures that the computed path is a descendant 
   of the intended root directory. If not, it refuses access, thereby mitigating symbolic link abuses.
*/

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io;
use std::sync::Arc;
use std::thread;

struct FileHandler {
    root: PathBuf,
}

impl FileHandler {
    fn new(root: PathBuf) -> Self {
        Self { root }
    }

    // Securely accesses a file by canonicalizing and then verifying that the resolved path 
    // is under the specified root directory.
    fn access(&self, relative: &str) -> io::Result<File> {
        let path = self.root.join(relative).canonicalize()?;
        if !path.starts_with(&self.root) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Access denied: resolved path is outside the allowed directory",
            ));
        }
        File::open(path)
    }
}

fn main() {
    // Assume this directory is the intended secure root.
    let base = PathBuf::from("/tmp/secure_dir");
    let handler = Arc::new(FileHandler::new(base));
    // Spawn threads to simulate concurrent access.
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let h = Arc::clone(&handler);
            thread::spawn(move || {
                // Only allow access to files strictly within the base directory.
                let _ = h.access("sample.txt");
            })
        })
        .collect();

    for t in threads {
        let _ = t.join();
    }
}