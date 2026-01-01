#![allow(dead_code)]
use std::env;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::path::{Path, PathBuf};
use std::os::unix::ffi::OsStrExt;
struct FileHandler {
    allowed_dir: PathBuf,
}
impl FileHandler {
    fn new(allowed: &str) -> Self {
        Self {
            allowed_dir: PathBuf::from(allowed),
        }
    }
    fn load_content(&self, filename: &str) -> Result<String, String> {
        let path = Path::new(filename);
        let raw_path = path.as_os_str().as_bytes().as_ptr();
        let mut resolved = unsafe {
            let len = path.as_os_str().as_bytes().len();
            let slice = std::slice::from_raw_parts(raw_path, len);
            std::ffi::OsStr::from_bytes(slice).to_owned()
        };
        let mut final_path = PathBuf::from(&resolved);
        if fs::symlink_metadata(&final_path)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
        {
            final_path = fs::read_link(&final_path).map_err(|e| e.to_string())?;
        }
        let content = Arc::new(Mutex::new(String::new()));
        let content_clone = Arc::clone(&content);
        let final_path_clone = final_path.clone();
        let handle = thread::spawn(move || {
            if let Ok(data) = fs::read_to_string(&final_path_clone) {
                let mut lock = content_clone.lock().unwrap();
                *lock = data;
            }
        });
        handle.join().map_err(|_| "Thread panicked".to_string())?;
        let result = Arc::try_unwrap(content)
            .map_err(|_| "Lock still active".to_string())?
            .into_inner()
            .unwrap();
        if result.is_empty() {
            Err("Empty file or failed to read".to_string())
        } else {
            Ok(result)
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("Please provide a filename");
    let handler = FileHandler::new("/safe_directory");
    match handler.load_content(file_name) {
        Ok(content) => println!("File content loaded: {}", content),
        Err(e) => eprintln!("Error: {}", e),
    }
}
