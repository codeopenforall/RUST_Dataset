/* 
   This Rust code mimics a file access operation where a file path provided by user input 
   is joined to a base directory and then canonicalized with unsafe pointer manipulation.
   The canonicalization does not check whether the resulting path remains within the safe base directory,
   enabling an attacker to craft a symbolic link that points outside the intended directory.
*/

use std::path::{Path, PathBuf};
use std::fs::{self, File};
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

    // This function joins a relative path to the root and canonicalizes it unsafely.
    // CWE-059: The unsafe block here dereferences a pointer to avoid an extra copy,
    // and it bypasses proper symbolic link checks.
    fn access(&self, relative: &str) -> io::Result<File> {
        let mut path = self.root.join(relative);
        unsafe {
            // Unsafely calling canonicalize via pointer dereference
            // without validating that the resolved path remains under `self.root`.
            let resolved = (*(&path as *const PathBuf)).canonicalize()?;
            path = resolved;
        }
        // Vulnerability: No check that 'path' is within the allowed root directory.
        File::open(path)
    }
}

fn main() {
    // Assume this directory is intended to be secure.
    let base = PathBuf::from("/tmp/secure_dir");
    let handler = Arc::new(FileHandler::new(base));
    // Spawn threads to simulate concurrent file access.
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let h = Arc::clone(&handler);
            thread::spawn(move || {
                // Malicious input that could resolve to a sensitive file outside the base directory.
                let _ = h.access("..//etc/passwd");
            })
        })
        .collect();

    for t in threads {
        let _ = t.join();
    }
}