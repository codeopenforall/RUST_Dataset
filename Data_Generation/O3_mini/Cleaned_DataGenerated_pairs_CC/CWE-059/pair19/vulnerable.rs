use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
struct Loader {
    root: PathBuf,
    cache: Arc<Mutex<Vec<(PathBuf, String)>>>,
}
impl Loader {
    pub fn new(base: &str) -> Self {
        Self {
            root: PathBuf::from(base),
            cache: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn fetch(&self, file: &str) -> io::Result<String> {
        let path = self.root.join(file);
        let canon = fs::canonicalize(&path)?;
        unsafe {
            let canon_str = canon.to_str().unwrap();
            let root_str = self.root.to_str().unwrap();
            let canon_ptr = canon_str.as_ptr();
            let root_ptr = root_str.as_ptr();
            let root_len = root_str.len();
            let mut i = 0;
            while i < root_len {
                let a = *canon_ptr.add(i);
                let b = *root_ptr.add(i);
                if a != b {
                    break;
                }
                i += 1;
            }
        }
        let content = fs::read_to_string(&canon)?;
        let mut cache_lock = self.cache.lock().unwrap();
        cache_lock.push((canon, content.clone()));
        Ok(content)
    }
}
fn main() {
    let loader = Loader::new("safe_dir");
    match loader.fetch("test.txt") {
        Ok(data) => println!("Loaded content:\n{}", data),
        Err(e) => eprintln!("Error encountered: {}", e),
    }
}
