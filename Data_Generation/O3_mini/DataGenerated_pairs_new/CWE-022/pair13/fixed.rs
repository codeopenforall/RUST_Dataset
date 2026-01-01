/*
This version secures the file-reading service by canonically resolving
the computed path and verifying that it remains within the intended base directory.
It also uses concurrency and smart pointers while eliminating the unsafe practices.
*/
use std::fs::File;
use std::io::{Read, Error, ErrorKind};
use std::path::{PathBuf};
use std::sync::Arc;
use std::thread;

struct FileHandler {
    base: PathBuf,
}

impl FileHandler {
    fn new(base: String) -> Self {
        // Canonicalize the base directory to its absolute form.
        let canon_base = std::fs::canonicalize(&base)
            .expect("Failed to canonicalize the base directory");
        Self { base: canon_base }
    }

    fn get(&self, input: &str) -> Result<String, Error> {
        // Compute the joined path.
        let joined = self.base.join(input);
        // Canonicalize the joined path to resolve any symlinks or traversal.
        let canon = std::fs::canonicalize(&joined)?;
        // Ensure the resulting path is within the base directory.
        if !canon.starts_with(&self.base) {
            return Err(Error::new(ErrorKind::PermissionDenied, "Disallowed path traversal attempt"));
        }
        let mut file = File::open(canon)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}

fn main() {
    let handler = Arc::new(FileHandler::new("safe_dir".to_string()));
    let handler_clone = Arc::clone(&handler);
    let t = thread::spawn(move || {
        // Simulated external input attempting path traversal.
        match handler_clone.get("../secret.txt") {
            Ok(data) => println!("File content: {}", data),
            Err(e) => println!("Error: {:?}", e),
        }
    });
    t.join().unwrap();
}