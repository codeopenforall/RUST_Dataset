use std::fs::File;
use std::io::{Read, Error as IoError, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
struct FileHandler {
    root: PathBuf,
}
impl FileHandler {
    fn new(root: PathBuf) -> Self {
        FileHandler { root }
    }
    fn fetch(&self, user_input: &str) -> Result<String, IoError> {
        let candidate = self.root.join(user_input);
        let candidate_canon = candidate.canonicalize()?;
        let base_canon = self.root.canonicalize()?;
        if !candidate_canon.starts_with(&base_canon) {
            return Err(IoError::new(
                ErrorKind::PermissionDenied,
                "access denied: outside permitted directory",
            ));
        }
        unsafe {
            let mut file = File::open(candidate_canon)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(content)
        }
    }
}
fn main() {
    let base_dir = PathBuf::from("./safe_dir");
    let handler = Arc::new(Mutex::new(FileHandler::new(base_dir)));
    let handler_clone = Arc::clone(&handler);
    let handle = thread::spawn(move || {
        let result = handler_clone.lock().unwrap().fetch("../Cargo.toml");
        match result {
            Ok(data) => {
                println!("Thread read (unexpected success): {}", &data[0..std::cmp::min(20, data.len())]);
            }
            Err(err) => {
                println!("Thread correctly failed: {}", err);
            }
        }
    });
    let res = handler.lock().unwrap().fetch("../Cargo.toml");
    match res {
        Ok(data) => {
            println!("Main thread read (unexpected success): {}", &data[0..std::cmp::min(20, data.len())]);
        }
        Err(err) => {
            println!("Main thread correctly failed: {}", err);
        }
    }
    handle.join().unwrap();
}
