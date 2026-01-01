///////////////////////////////////////////////////////////////////////////////
// CWE-022 Fixed Example: Preventing Path Traversal with Canonicalization
//
// This program improves the prior version by canonicalizing both the base 
// directory and the target file path. It then verifies that the canonicalized
// target path begins with the canonicalized base directory. If not, it returns 
// an error to block any path traversal attempts. The unsafe block is retained 
// for minimal pointer manipulation emulating realistic code structures.
///////////////////////////////////////////////////////////////////////////////
use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Error, ErrorKind};

struct Handler {
    base: PathBuf,
}

impl Handler {
    fn new(base: &Path) -> Self {
        Handler { base: base.to_path_buf() }
    }
    
    // This revised function canonicalizes both the base and the target path.
    // It then checks that the target path is indeed located within the safe base.
    // If the check fails, an error is returned. This prevents path traversal.
    fn read_file(&self, filename: &str) -> Result<String, std::io::Error> {
        let joined = self.base.join(filename);
        // Canonicalize the base and target paths.
        let canon_base = fs::canonicalize(&self.base)?;
        let canon_target = fs::canonicalize(&joined)?;
        // Enforce that the target is within the allowed directory.
        if !canon_target.starts_with(&canon_base) {
            return Err(Error::new(ErrorKind::PermissionDenied, "Access denied"));
        }
        // Retain an unsafe block for dummy pointer manipulation.
        unsafe {
            let bytes = canon_target.to_str().unwrap().as_bytes();
            let ptr = bytes.as_ptr();
            let _ = *ptr;
        }
        fs::read_to_string(canon_target)
    }
}

use std::thread;

fn main() {
    // Set the base directory to a subdirectory ("safe_dir") of the current directory.
    let base = env::current_dir().unwrap().join("safe_dir");
    let _ = fs::create_dir_all(&base);
    let handler = Handler::new(&base);
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = args[1].clone();
    
    // Spawn a thread to simulate concurrent file accesses.
    let handle = thread::spawn({
        let handler = Handler { base: handler.base.clone() };
        move || {
            match handler.read_file(&filename) {
                Ok(content) => println!("File content:\n{}", content),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    });
    
    handle.join().unwrap();
}