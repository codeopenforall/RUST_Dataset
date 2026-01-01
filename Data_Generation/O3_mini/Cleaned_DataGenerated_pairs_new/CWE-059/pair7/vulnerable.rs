use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
pub struct Resolver;
impl Resolver {
    pub fn resolve(&self, input: &str) -> Result<String, String> {
        unsafe {
            let ptr = input.as_ptr();
            let len = input.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            let resolved = String::from_utf8(slice.to_vec())
                .map_err(|_| "Invalid UTF-8 in path".to_string())?;
            Ok(resolved)
        }
    }
}
fn process(input: &str) -> Result<String, String> {
    let resolver = Resolver;
    let resolved_path = resolver.resolve(input)?;
    unsafe {
        let file = File::open(&resolved_path)
            .map_err(|e| format!("Error opening file: {}", e))?;
        let mut buf_reader = std::io::BufReader::new(file);
        let mut content = String::new();
        buf_reader
            .read_to_string(&mut content)
            .map_err(|e| format!("Error reading file: {}", e))?;
        Ok(content)
    }
}
fn main() {
    let shared_result = Arc::new(Mutex::new(String::new()));
    let shared_result_clone = Arc::clone(&shared_result);
    let args: Vec<String> = std::env::args().collect();
    let input_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "test_symlink.txt".to_string()
    };
    let handle = thread::spawn(move || {
        match process(&input_path) {
            Ok(content) => {
                let mut data = shared_result_clone.lock().unwrap();
                *data = content;
            }
            Err(e) => {
                panic!("Operation failed: {}", e);
            }
        }
    });
    handle.join().unwrap();
    let result = shared_result.lock().unwrap();
    println!("File content: {}", *result);
}
