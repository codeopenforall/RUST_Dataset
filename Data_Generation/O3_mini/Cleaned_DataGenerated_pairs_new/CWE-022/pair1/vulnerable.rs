use std::fs::File;
use std::io::{Read, Error as IoError};
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
        let joined = self.root.join(user_input);
        unsafe {
            let mut file = File::open(joined)?;
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
                println!("Thread completed read: {}", &data[0..std::cmp::min(20, data.len())]);
            }
            Err(err) => {
                eprintln!("Thread got error: {}", err);
            }
        }
    });
    let res = handler.lock().unwrap().fetch("../Cargo.toml");
    match res {
        Ok(data) => {
            println!("Main thread read: {}", &data[0..std::cmp::min(20, data.len())]);
        }
        Err(err) => {
            eprintln!("Main thread error: {}", err);
        }
    }
    handle.join().unwrap();
}
