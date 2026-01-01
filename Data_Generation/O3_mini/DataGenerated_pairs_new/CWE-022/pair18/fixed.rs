////////////////////////////////////////////////////////////
// Corrected Rust code that mitigates CWE-022 path traversal
////////////////////////////////////////////////////////////
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::thread;

struct FileHandler {
    base: PathBuf,
}

impl FileHandler {
    fn new(base: &str) -> Self {
        FileHandler {
            base: PathBuf::from(base),
        }
    }

    // Securely compute file path: canonicalize both the base directory and the candidate path,
    // and ensure that the candidate path is within the base directory.
    fn get_secure_path(&self, input: &str) -> io::Result<PathBuf> {
        let candidate = self.base.join(input).canonicalize()?;
        let base_canonical = self.base.canonicalize()?;
        if candidate.starts_with(&base_canonical) {
            Ok(candidate)
        } else {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Path traversal attempt detected",
            ))
        }
    }

    // Reads the file content concurrently after validating the file path.
    // Retains an unsafe block to simulate low-level operations, as seen in realistic code.
    fn load(&self, input: &str) -> io::Result<String> {
        let secure_path = self.get_secure_path(input)?;
        let path_clone = secure_path.clone();
        let handle = thread::spawn(move || fs::read_to_string(&path_clone));
        let content = handle.join().unwrap()?;
        // UNSAFE: Using raw pointer arithmetic similar to the vulnerable implementation.
        let result = unsafe {
            let ptr = content.as_ptr();
            let len = content.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            String::from_utf8_lossy(slice).into_owned()
        };
        Ok(result)
    }
}

fn main() {
    // Base directory is assumed to contain only authorized files.
    let handler = FileHandler::new("./data");
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <relative_file>", args[0]);
        std::process::exit(1);
    }
    let input = &args[1];
    match handler.load(input) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}