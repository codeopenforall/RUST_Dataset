///////////////////////////////////////////////////////////////////////////////
// CWE-022 Vulnerable Example: Path Traversal Issue using unsafe, concurrency,
// and naive PathBuf::join without canonicalization.
//
// This program defines a handler that reads a file from a safe directory by
// naively joining the base directory with a user-supplied filename. No
// canonicalization or prefix checks are done, allowing path traversal.
// An unsafe block is used for trivial pointer manipulation. A concurrent
// thread is spawned to simulate realistic usage patterns.
///////////////////////////////////////////////////////////////////////////////
use std::env;
use std::path::{Path, PathBuf};
use std::fs;

struct Handler {
    base: PathBuf,
}

impl Handler {
    fn new(base: &Path) -> Self {
        Handler { base: base.to_path_buf() }
    }
    
    // This function is vulnerable because it simply joins the base directory
    // with the user-supplied filename without canonicalization or checking for
    // directory traversal sequences like "../". An unsafe block is used to perform
    // trivial pointer arithmetic (emulating unsafe practices in real-world code).
    fn read_file(&self, filename: &str) -> Result<String, std::io::Error> {
        // Vulnerability: naive join without canonicalization
        let target = self.base.join(filename);
        // Unsafe block for dummy pointer operation.
        unsafe {
            let bytes = target.to_str().unwrap().as_bytes();
            let ptr = bytes.as_ptr();
            // Dummy operation: read first byte unsafely.
            let _ = *ptr;
        }
        fs::read_to_string(target)
    }
}

use std::thread;

fn main() {
    // Set the base directory to a subdirectory ("safe_dir") of the current directory,
    // which is intended to be the only allowed area for file access.
    let base = env::current_dir().unwrap().join("safe_dir");
    // Ensure the safe directory exists.
    let _ = fs::create_dir_all(&base);
    let handler = Handler::new(&base);
    
    // Get the filename from command line arguments.
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