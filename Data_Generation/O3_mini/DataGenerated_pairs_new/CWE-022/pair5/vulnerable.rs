////////////////// Vulnerable Version //////////////////
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

pub struct Loader {
    base_dir: PathBuf,
}

impl Loader {
    pub fn new(base: &str) -> Self {
        Loader { base_dir: PathBuf::from(base) }
    }

    // Loads a file given a relative path.
    pub fn load_file(&self, name: &str) -> Result<String, String> {
        // FLAW: Naively joining user input with the base directory without proper normalization.
        let file_path = self.base_dir.join(name); // <-- Vulnerability: CWE-022 (Path Traversal)

        // Open and read file contents.
        let mut file = File::open(&file_path)
            .map_err(|e| format!("File open error: {}", e))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Read error: {}", e))?;
        // Unsafe conversion from bytes to str (for demonstration purposes).
        unsafe {
            let s = std::str::from_utf8_unchecked(&buffer);
            Ok(s.to_string())
        }
    }
}

fn main() {
    let loader = Arc::new(Loader::new("./data"));
    let loader_clone = Arc::clone(&loader);
    let handle = thread::spawn(move || {
        match loader_clone.load_file("example.txt") {
            Ok(content) => println!("Thread file content: {}", content),
            Err(e) => println!("Thread error: {}", e),
        }
    });
    handle.join().unwrap();
    println!("Main thread finished execution.");
}