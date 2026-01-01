//////////////////////////////////////////////////
// Secure Implementation: Mitigation for CWE-022 Path Traversal
//////////////////////////////////////////////////
#![allow(unused)]
use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use std::path::{PathBuf, Component};
use std::sync::{Arc, Mutex};
use std::thread;

struct FileHandler {
    base: PathBuf,
}

impl FileHandler {
    fn new(root: &str) -> Self {
        FileHandler { base: PathBuf::from(root) }
    }

    // Reads a file by securely joining the base directory with validated user input.
    // This version prevents path traversal by disallowing parent directory components.
    fn load(&self, input: &str) -> Result<String, Error> {
        // Validate input by rejecting any parent directory components.
        let user_path = PathBuf::from(input);
        for comp in user_path.components() {
            if let Component::ParentDir = comp {
                return Err(Error::new(ErrorKind::InvalidInput, "Path traversal is not allowed"));
            }
        }
        // Securely join the base directory with the validated input.
        let joined = self.base.join(&user_path)
            .canonicalize()?;
        if !joined.starts_with(&self.base) {
            return Err(Error::new(ErrorKind::InvalidInput, "Resolved path is outside the designated directory"));
        }
        
        // Use unsafe pointer manipulation in a controlled manner.
        let joined_str = joined.to_str().ok_or_else(|| Error::new(ErrorKind::Other, "Invalid path conversion"))?;
        let mut safe_path = PathBuf::new();
        unsafe {
            let raw_ptr = joined_str.as_ptr();
            let len = joined_str.len();
            let slice = std::slice::from_raw_parts(raw_ptr, len);
            let validated = std::str::from_utf8(slice).map_err(|_| Error::new(ErrorKind::Other, "UTF8 error"))?;
            safe_path = PathBuf::from(validated);
        }

        // Concurrently read file content using a spawned thread.
        let path_clone = safe_path.clone();
        let data = Arc::new(Mutex::new(String::new()));
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
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
    // Valid input within the allowed directory.
    let filename = "allowed.txt";
    match handler.load(filename) {
        Ok(content) => println!("File content: {}", content),
        Err(e) => eprintln!("Error reading file: {:?}", e),
    }
}