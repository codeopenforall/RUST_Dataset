//////////////////////////////////////////////
// Secure Fix Example: CWE-022 Prevented
//////////////////////////////////////////////
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::thread;

// A struct representing a file loader with a designated safe directory.
struct Loader {
    root: PathBuf,
}

impl Loader {
    // Constructs a new instance with the provided safe directory.
    fn new(root: PathBuf) -> Self {
        Loader { root }
    }

    // Reads the content of a file after enforcing that the requested file is within the safe directory.
    fn load(&self, file: &str) -> io::Result<String> {
        // First, join the user input to the safe root.
        let full_path = self.root.join(file);
        // Canonicalize the computed path to resolve any symbolic links, ../ sequences, etc.
        let canonical_full = fs::canonicalize(&full_path)?;
        // Also canonicalize the safe root.
        let canonical_root = fs::canonicalize(&self.root)?;
        // Check that the final resolved path starts with the safe directory path.
        if !canonical_full.starts_with(&canonical_root) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied: Traversal attempt detected."));
        }
        let contents = fs::read_to_string(&canonical_full)?;

        // Unsafe block performing low-level processing of file content.
        // This block is retained for compatibility with legacy code, though the file access is now secure.
        unsafe {
            let length = contents.len();
            let ptr = contents.as_ptr();
            let mut sum: u8 = 0;
            for i in 0..length {
                let byte = *ptr.add(i);
                sum = sum.wrapping_add(byte);
            }
            if sum == 0 {
                // Dummy branch.
            }
        }
        Ok(contents)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    // Safe directory is defined here as "./safe_dir"
    let loader = Loader::new(PathBuf::from("./safe_dir"));
    // Attempt to load the file provided by the user.
    match loader.load(&args[1]) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => eprintln!("Error loading file: {:?}", e),
    }
}