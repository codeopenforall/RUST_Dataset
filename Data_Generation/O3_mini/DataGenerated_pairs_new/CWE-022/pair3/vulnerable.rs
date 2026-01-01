//////////////////////////////////////////////////
// Vulnerability Implementation: CWE-022 (Path Traversal)
//////////////////////////////////////////////////
#![allow(unused)]
use std::fs::File;
use std::io::{Read, Error};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

struct FileHandler {
    base: PathBuf,
}

impl FileHandler {
    fn new(root: &str) -> Self {
        FileHandler { base: PathBuf::from(root) }
    }

    // Reads a file by naively joining the base directory with user input.
    // Unsafe block is used for raw pointer manipulation to mimic complex real-world code.
    fn load(&self, input: &str) -> Result<String, Error> {
        // Naively append the user provided input to the base directory without sanitization.
        let mut full = self.base.clone();
        full.push(input); // Vulnerability: no validation or canonicalization performed here.

        // Unsafely manipulate the path string.
        let path_str = full.to_str().ok_or_else(|| Error::from_raw_os_error(22))?;
        unsafe {
            // Simulate unsafe raw pointer usage found in some low-level routines.
            let raw_ptr = path_str.as_ptr();
            let len = path_str.len();
            let slice = std::slice::from_raw_parts(raw_ptr, len);
            let manipulated = std::str::from_utf8(slice).map_err(|_| Error::from_raw_os_error(22))?;
            full = PathBuf::from(manipulated);
        }

        // Concurrently read file content using a spawned thread.
        let path_clone = full.clone();
        let data = Arc::new(Mutex::new(String::new()));
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // Attempt to open the file; if the file is outside intended directory
            // and doesn't exist, this may return a OS error.
            let mut file = File::open(&path_clone).expect("unable to open file");
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).expect("unable to read file");
            let mut d = data_clone.lock().unwrap();
            *d = buffer;
        });
        handle.join().unwrap();
        let result = Arc::try_unwrap(data).unwrap().into_inner().unwrap();
        Ok(result)
    }
}

fn main() {
    let handler = FileHandler::new("/safe/dir");
    // Malicious input attempting path traversal.
    let filename = "../etc/passwd";
    match handler.load(filename) {
        Ok(content) => println!("File content: {}", content),
        Err(e) => eprintln!("Error reading file: {:?}", e),
    }
}