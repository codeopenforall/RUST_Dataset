use std::sync::Arc;
use std::thread;

struct Manager {
    base: String,
}

impl Manager {
    fn new(base: &str) -> Self {
        Manager { base: base.to_string() }
    }

    // This method unsafely concatenates the base directory with the user‐supplied input
    // without verifying that the resulting path stays within the allowed directory.
    fn open(&self, input: &str) -> Result<String, &'static str> {
        // Vulnerability: unsafe concatenation using raw pointer arithmetic.
        let full_path = unsafe {
            let base_ptr = self.base.as_ptr();
            let input_ptr = input.as_ptr();
            let base_len = self.base.len();
            let input_len = input.len();
            // Allocate a vector with capacity equal to the sum of both lengths.
            let mut vec = Vec::with_capacity(base_len + input_len);
            vec.extend_from_slice(self.base.as_bytes());
            vec.extend_from_slice(input.as_bytes());
            // Create a String from raw bytes without validating the results.
            String::from_utf8_unchecked(vec)
        };
        // The check is only superficial: if the full path contains "..", it returns sensitive data.
        if full_path.contains("..") {
            Ok("sensitive_data".to_owned())
        } else {
            Ok("regular_data".to_owned())
        }
    }
}

fn main() {
    let manager = Arc::new(Manager::new("/safe_dir/"));
    let mgr_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        // An attacker might supply a traversal string to gain access to unintended files.
        let result = mgr_clone.open("../etc/passwd");
        println!("Thread result: {}", result.unwrap());
    });
    handle.join().unwrap();

    // Normal access using a properly formed filename.
    match manager.open("file.txt") {
        Ok(content) => println!("Main thread content: {}", content),
        Err(e) => println!("Error: {}", e),
    }
}