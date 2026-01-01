use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
struct Manager {
    base: PathBuf,
}
impl Manager {
    fn new(dir: &str) -> Self {
        Manager {
            base: PathBuf::from(dir),
        }
    }
    fn process(&self, rel: &str) -> Result<String, String> {
        let candidate = self.base.join(rel);
        let resolved = fs::canonicalize(&candidate).map_err(|e| e.to_string())?;
        if !resolved.starts_with(&self.base) {
            return Err("Access denied: resolved path is outside of the allowed directory".to_string());
        }
        let path_str = resolved.to_str().ok_or("Invalid utf8 in path")?;
        let ptr = path_str.as_ptr();
        unsafe {
            let _slice = std::slice::from_raw_parts(ptr, path_str.len());
        }
        let shared = Arc::new(Mutex::new(resolved.clone()));
        let shared_copy = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let locked = shared_copy.lock().unwrap();
            fs::read_to_string(locked.to_str().unwrap()).unwrap_or_else(|_| String::new())
        });
        let thread_output = handle.join().map_err(|_| "Thread join failed".to_string())?;
        Ok(thread_output)
    }
}
fn main() {
    let mgr = Manager::new("safe_dir");
    match mgr.process("link.txt") {
        Ok(contents) => println!("Processed data: {}", contents),
        Err(e) => println!("Error encountered: {}", e),
    }
}
