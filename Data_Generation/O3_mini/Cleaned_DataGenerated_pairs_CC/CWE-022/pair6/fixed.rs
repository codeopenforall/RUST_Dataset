use std::fs;
use std::path::{PathBuf, Path};
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CString, CStr};
pub struct FileManager {
    base: PathBuf,
}
impl FileManager {
    pub fn new(base: PathBuf) -> Self {
        FileManager { base }
    }
    pub fn open_file(&self, user_input: &str) -> Result<String, String> {
        let joined_path = self.base.join(user_input);
        let canonical_base = fs::canonicalize(&self.base)
            .map_err(|e| format!("Error canonicalizing base: {:?}", e))?;
        let canonical_path = fs::canonicalize(&joined_path)
            .map_err(|e| format!("Error canonicalizing path: {:?}", e))?;
        if !canonical_path.starts_with(&canonical_base) {
            return Err("Access to the requested file is not allowed".to_string());
        }
        unsafe {
            let path_str = canonical_path.to_str().ok_or_else(|| "Invalid path".to_string())?;
            let c_path = CString::new(path_str).map_err(|_| "Failed to convert path")?;
            let raw_ptr = c_path.as_ptr();
            let file_path = CStr::from_ptr(raw_ptr)
                .to_str()
                .map_err(|_| "Conversion error")?;
            let content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
            Ok(content)
        }
    }
}
fn main() {
    let base_dir = PathBuf::from("./data");
    let manager = Arc::new(Mutex::new(FileManager::new(base_dir)));
    let handles: Vec<_> = (0..1).map(|_| {
        let mgr = Arc::clone(&manager);
        thread::spawn(move || {
            let fm = mgr.lock().unwrap();
            match fm.open_file("test.txt") {
                Ok(content) => println!("Content: {}", content),
                Err(e) => eprintln!("Error: {}", e),
            }
        })
    }).collect();
    for handle in handles {
        handle.join().unwrap();
    }
}
