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

    // This method attempts to load a file using an unsafe conversion from a Rust string to a C string.
    // It does not recheck that the file path has not been altered (e.g. via a malicious symlink),
    // which can allow unintended files to be accessed (CWE-059).
    pub fn load(&self, path: &Path) -> Result<String, String> {
        let full_path = self.base.join(path);
        // Unsafe conversion of the full path to a C-compatible string.
        let cstr = CString::new(full_path.to_str().ok_or("Invalid path")?).map_err(|e| e.to_string())?;
        unsafe {
            let ptr = cstr.as_ptr();
            // Convert back from C string to Rust string without revalidating the path.
            let reparsed = CStr::from_ptr(ptr).to_str().map_err(|e| e.to_string())?;
            // A race window exists here.
            if fs::metadata(Path::new(reparsed)).is_err() {
                return Err("File not found".into());
            }
        }
        // The file is opened without further checks on symlink resolution.
        let mut file = File::open(full_path).map_err(|e| e.to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| e.to_string())?;
        Ok(content)
    }
}

fn main() {
    // The base directory where files are expected to reside.
    let base = PathBuf::from("/tmp/allowed");
    let resolver = FileResolver::new(base);
    let resolver_arc = Arc::new(resolver);
    let resolver_thread = Arc::clone(&resolver_arc);
    let handle = thread::spawn(move || {
        // Expected input: a file named "test.txt" may be a symlink.
        match resolver_thread.load(Path::new("test.txt")) {
            Ok(content) => println!("{}", content),
            Err(err) => println!("Error: {}", err),
        }
    });
    handle.join().unwrap();
}