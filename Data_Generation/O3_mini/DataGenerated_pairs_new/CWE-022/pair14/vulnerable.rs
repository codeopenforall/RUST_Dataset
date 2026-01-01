//////////////////////////////
// Vulnerability demonstration code
//////////////////////////////
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

struct FileHandler {
    base: PathBuf,
}

impl FileHandler {
    fn new(base: &str) -> Self {
        Self { base: PathBuf::from(base) }
    }

    // This function naively joins the user-supplied input with the base directory.
    // It does not canonicalize or verify the resulting path, exposing a CWE-022 (Path Traversal) vulnerability.
    fn get_content(&self, input: &str) -> std::io::Result<String> {
        // Vulnerable: using naive join without canonicalization allow path traversal
        let path = self.base.join(input);
        // An unsafe block introduced for demonstration purposes.
        unsafe {
            let s = path.as_os_str().to_str().unwrap();
            // Arbitrary pointer dereference (harmless in this context, but illustrative)
            let ptr = s.as_ptr();
            let _ = *ptr;
        }
        fs::read_to_string(path)
    }
}

fn main() {
    // Expecting a command-line argument defining the file path to read.
    let args: Vec<String> = env::args().collect();
    let handler = Arc::new(FileHandler::new("./data"));
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let input = args[1].clone();
    let handler_clone = handler.clone();
    let thread_handle = thread::spawn(move || {
        match handler_clone.get_content(&input) {
            Ok(c) => println!("Content:\n{}", c),
            Err(e) => println!("Error encountered: {}", e),
        }
    });
    let _ = thread_handle.join();
}