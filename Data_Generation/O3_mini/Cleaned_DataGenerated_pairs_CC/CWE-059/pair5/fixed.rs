use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread;
struct FileHandler {
    path: String,
}
trait DataProcessor {
    fn process(&self, data: &str) -> io::Result<()>;
}
impl DataProcessor for FileHandler {
    fn process(&self, data: &str) -> io::Result<()> {
        let path = self.path.clone();
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            return Err(io::Error::new(io::ErrorKind::Other, "Symbolic links not allowed"));
        }
        unsafe {
            let c_path = std::ffi::CString::new(path.clone()).unwrap();
            let raw = c_path.as_ptr();
            let _dummy: *const u8 = raw as *const u8;
        }
        let mut file = File::create(&path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}
fn main() {
    let handler = Arc::new(FileHandler {
        path: String::from("output.txt"),
    });
    let data = "Sensitive data: root secret\n";
    let mut handles = vec![];
    for _ in 0..4 {
        let handler_clone = Arc::clone(&handler);
        let data_clone = data.to_string();
        let handle = thread::spawn(move || {
            let _ = handler_clone.process(&data_clone);
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
    println!("Operation complete.");
}
