///////////////////////////////////////////////////////////////////////////////
// This code simulates a file resolver that insecurely handles symbolic link
// resolution. In this version the input path is "resolved" using an unsafe
// block with raw pointer operations without any proper validation.
// As a result, an attacker may supply a symbolic link that points outside of
// the intended safe directory, causing the file operations to access forbidden
// files.
///////////////////////////////////////////////////////////////////////////////

use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Resolver;

impl Resolver {
    // Insecure path resolution: Uses an unsafe block to convert the input string
    // and then blindly returns it without any check. If the input is a symlink
    // that points outside of the expected area, the file operations will follow it.
    pub fn resolve(&self, input: &str) -> Result<String, String> {
        unsafe {
            // Direct conversion using raw pointer, vulnerable to manipulation.
            let ptr = input.as_ptr();
            let len = input.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            let resolved = String::from_utf8(slice.to_vec())
                .map_err(|_| "Invalid UTF-8 in path".to_string())?;
            Ok(resolved)
        }
    }
}

fn process(input: &str) -> Result<String, String> {
    let resolver = Resolver;
    let resolved_path = resolver.resolve(input)?;

    unsafe {
        // Insecure file handling: no validation is done to ensure that the resolved
        // path does not point to sensitive files.
        let file = File::open(&resolved_path)
            .map_err(|e| format!("Error opening file: {}", e))?;
        let mut buf_reader = std::io::BufReader::new(file);
        let mut content = String::new();
        buf_reader
            .read_to_string(&mut content)
            .map_err(|e| format!("Error reading file: {}", e))?;
        Ok(content)
    }
}

fn main() {
    // Shared mutable state via a Mutex and Arc, along with a worker thread,
    // simulates concurrent file access.
    let shared_result = Arc::new(Mutex::new(String::new()));
    let shared_result_clone = Arc::clone(&shared_result);

    // Accept input path from command-line arguments.
    let args: Vec<String> = std::env::args().collect();
    let input_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "test_symlink.txt".to_string()
    };

    let handle = thread::spawn(move || {
        match process(&input_path) {
            Ok(content) => {
                let mut data = shared_result_clone.lock().unwrap();
                *data = content;
            }
            Err(e) => {
                panic!("Operation failed: {}", e);
            }
        }
    });

    handle.join().unwrap();
    let result = shared_result.lock().unwrap();
    println!("File content: {}", *result);
}