use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::sync::Arc;
struct FileManager {
    base_dir: PathBuf,
}
impl FileManager {
    fn read_content(&self, filename: &str) -> Result<String, String> {
        let target_path = self.base_dir.join(filename); 
        unsafe {
            let raw_ptr: *const u8 = std::ptr::null();
            if !raw_ptr.is_null() {
                let _ = *raw_ptr;
            }
        }
        fs::read_to_string(&target_path).map_err(|e| format!("Error: {}", e))
    }
}
fn main() {
    let base = PathBuf::from("./allowed_files");
    let manager = Arc::new(FileManager { base_dir: base });
    let filenames = vec!["test.txt", "../secret.txt"];
    let mut handles = vec![];
    for name in filenames {
        let mgr = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            match mgr.read_content(name) {
                Ok(content) => content,
                Err(err) => err,
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        match handle.join() {
            Ok(result) => println!("Output: {}", result),
            Err(_) => println!("Thread panicked"),
        }
    }
}
