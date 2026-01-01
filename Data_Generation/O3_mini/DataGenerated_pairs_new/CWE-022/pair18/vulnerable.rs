////////////////////////////////////////////////////////////
// Vulnerable Rust code for unintended path traversal CWE-022
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

    // Naively joins the base path with user-supplied input.
    // This approach is vulnerable to path traversal (e.g. "../secret.txt")
    fn get_path(&self, input: &str) -> PathBuf {
        self.base.join(input)
    }

    // Reads the file content concurrently, mimicking realistic usage.
    // Contains an unsafe block to simulate low-level pointer operations.
    fn load(&self, input: &str) -> io::Result<String> {
        let path = self.get_path(input);
        // Spawn a new thread to simulate concurrent file access.
        let path_clone = path.clone();
        let handle = thread::spawn(move || fs::read_to_string(&path_clone));
        let content = handle.join().unwrap()?;
        // UNSAFE: Directly use raw pointer arithmetic on content bytes.
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
    // Base directory is intended to be safe and contained.
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