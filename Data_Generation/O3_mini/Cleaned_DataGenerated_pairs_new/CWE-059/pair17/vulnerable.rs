use std::fs::File;
use std::io::{Read, Result};
use std::sync::Arc;
use std::thread;
use std::ffi::CStr;
struct Loader;
impl Loader {
    fn fetch(&self, path: &str) -> Result<String> {
        let _ = unsafe {
            let raw_ptr = path.as_ptr();
            CStr::from_ptr(raw_ptr as *const i8)
        };
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}
fn main() {
    let loader = Arc::new(Loader);
    let mut threads = Vec::new();
    let config_path = "config.cfg"; 
    for _ in 0..4 {
        let loader_clone = Arc::clone(&loader);
        let path_clone = config_path.to_string();
        threads.push(thread::spawn(move || {
            let _ = loader_clone.fetch(&path_clone);
        }));
    }
    for th in threads {
        let _ = th.join();
    }
    println!("Operation completed.");
}
