use std::fs::File;
use std::io::{Read, Error};
use std::path::{PathBuf};
use std::sync::Arc;
use std::thread;
struct FileHandler {
    base: PathBuf,
}
impl FileHandler {
    fn new(base: String) -> Self {
        Self { base: PathBuf::from(base) }
    }
    fn get(&self, input: &str) -> Result<String, Error> {
        let file_path = self.base.join(input);
        unsafe {
            let ptr: *const PathBuf = &self.base as *const _;
            std::ptr::read(ptr);
        }
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}
fn main() {
    let handler = Arc::new(FileHandler::new("safe_dir".to_string()));
    let handler_clone = Arc::clone(&handler);
    let t = thread::spawn(move || {
        match handler_clone.get("../secret.txt") {
            Ok(data) => println!("File content: {}", data),
            Err(e) => println!("Error: {:?}", e),
        }
    });
    t.join().unwrap();
}
