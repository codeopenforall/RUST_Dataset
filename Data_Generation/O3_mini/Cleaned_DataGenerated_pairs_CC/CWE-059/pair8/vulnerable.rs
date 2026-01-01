use std::ffi::{CString, CStr};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
pub struct FileResolver {
    base: PathBuf,
}
impl FileResolver {
    pub fn new(base: PathBuf) -> Self {
        FileResolver { base }
    }
    pub fn load(&self, path: &Path) -> Result<String, String> {
        let full_path = self.base.join(path);
        let cstr = CString::new(full_path.to_str().ok_or("Invalid path")?).map_err(|e| e.to_string())?;
        unsafe {
            let ptr = cstr.as_ptr();
            let reparsed = CStr::from_ptr(ptr).to_str().map_err(|e| e.to_string())?;
            if fs::metadata(Path::new(reparsed)).is_err() {
                return Err("File not found".into());
            }
        }
        let mut file = File::open(full_path).map_err(|e| e.to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| e.to_string())?;
        Ok(content)
    }
}
fn main() {
    let base = PathBuf::from("/tmp/allowed");
    let resolver = FileResolver::new(base);
    let resolver_arc = Arc::new(resolver);
    let resolver_thread = Arc::clone(&resolver_arc);
    let handle = thread::spawn(move || {
        match resolver_thread.load(Path::new("test.txt")) {
            Ok(content) => println!("{}", content),
            Err(err) => println!("Error: {}", err),
        }
    });
    handle.join().unwrap();
}
