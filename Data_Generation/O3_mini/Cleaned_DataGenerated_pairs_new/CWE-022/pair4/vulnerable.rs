use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, Read};
use std::sync::Arc;
use std::thread;
struct FileHandler {
    base_dir: PathBuf,
}
impl FileHandler {
    fn new(base: &str) -> Self {
        FileHandler {
            base_dir: PathBuf::from(base),
        }
    }
    fn read_file(&self, input: &str) -> io::Result<String> {
        let target_path = self.base_dir.join(input);
        let mut file = unsafe {
            File::open(&target_path)?
        };
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}
fn main() {
    let handler = Arc::new(FileHandler::new("./data"));
    let input = std::env::args().nth(1).unwrap_or_else(|| "default.txt".to_string());
    let handler_clone = Arc::clone(&handler);
    let worker = thread::spawn(move || {
        match handler_clone.read_file(&input) {
            Ok(text) => println!("File content:\n{}", text),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    });
    worker.join().unwrap();
}
