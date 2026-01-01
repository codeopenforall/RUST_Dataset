#![allow(unused)]
use std::fs::File;
use std::io::{Read, Error};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
struct FileHandler {
    base: PathBuf,
}
impl FileHandler {
    fn new(root: &str) -> Self {
        FileHandler { base: PathBuf::from(root) }
    }
    fn load(&self, input: &str) -> Result<String, Error> {
        let mut full = self.base.clone();
        full.push(input); 
        let path_str = full.to_str().ok_or_else(|| Error::from_raw_os_error(22))?;
        unsafe {
            let raw_ptr = path_str.as_ptr();
            let len = path_str.len();
            let slice = std::slice::from_raw_parts(raw_ptr, len);
            let manipulated = std::str::from_utf8(slice).map_err(|_| Error::from_raw_os_error(22))?;
            full = PathBuf::from(manipulated);
        }
        let path_clone = full.clone();
        let data = Arc::new(Mutex::new(String::new()));
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut file = File::open(&path_clone).expect("unable to open file");
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).expect("unable to read file");
            let mut d = data_clone.lock().unwrap();
            *d = buffer;
        });
        handle.join().unwrap();
        let result = Arc::try_unwrap(data).unwrap().into_inner().unwrap();
        Ok(result)
    }
}
fn main() {
    let handler = FileHandler::new("/safe/dir");
    let filename = "../etc/passwd";
    match handler.load(filename) {
        Ok(content) => println!("File content: {}", content),
        Err(e) => eprintln!("Error reading file: {:?}", e),
    }
}
