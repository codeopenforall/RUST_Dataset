use std::fs;
use std::ffi::{CString, CStr};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

pub struct FileManager {
    base: PathBuf,
}

impl FileManager {
    pub fn new(base: &Path) -> Self {
        Self {
            base: base.to_path_buf(),
        }
    }

    pub fn open_file(&self, name: &str) -> std::io::Result<String> {
        let mut full_path = self.base.join(name);
        // If the file exists, perform an unsafe conversion of the path,
        // simulating a careless use of low-level string manipulation.
        if full_path.exists() {
            unsafe {
                let c_str = CString::new(full_path.to_string_lossy().as_bytes()).unwrap();
                let ptr = c_str.as_ptr();
                let recovered = CStr::from_ptr(ptr).to_string_lossy().into_owned();
                full_path = PathBuf::from(recovered);
            }
        }
        // FLAW: No verification that the resolved file is within the intended directory.
        fs::read_to_string(&full_path)
    }
}

fn main() {
    // The base directory intended to be safe.
    let base = Path::new("./safe_dir");
    let manager = FileManager::new(&base);
    let shared_manager = Arc::new(manager);
    let handle_manager = Arc::clone(&shared_manager);

    let handle = thread::spawn(move || {
        // Attempting to read a file using directory traversal (e.g. "../")
        let result = handle_manager.open_file("../etc/passwd");
        match result {
            Ok(content) => println!("File content: {}", &content[..std::cmp::min(content.len(), 100)]),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    });
    handle.join().unwrap();
}