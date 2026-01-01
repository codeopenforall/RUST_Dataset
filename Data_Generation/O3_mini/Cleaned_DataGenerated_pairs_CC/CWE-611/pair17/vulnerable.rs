use std::ffi::{CString, CStr};
use std::thread;
use std::sync::{Arc, Mutex};
pub struct Parser {
    state: Arc<Mutex<String>>,
}
impl Parser {
    pub fn new() -> Self {
        Parser {
            state: Arc::new(Mutex::new(String::new())),
        }
    }
    pub fn parse(&self, input: &str) -> Result<String, String> {
        let mut result = input.to_string();
        if input.contains("&xxe;") {
            let filename = "vulnerable.txt";
            let state_clone = Arc::clone(&self.state);
            let handle = thread::spawn(move || {
                unsafe {
                    let file_data = unsafe_fetch(filename);
                    if let Ok(mut data) = state_clone.lock() {
                        *data = file_data;
                    }
                }
            });
            handle.join().map_err(|_| "Thread panicked".to_string())?;
            let file_content = self.state.lock().map_err(|_| "Lock error".to_string())?.clone();
            result = result.replace("&xxe;", &file_content);
        }
        Ok(result)
    }
}
unsafe fn unsafe_fetch(file: &str) -> String {
    let c_filename = CString::new(file).unwrap();
    let ptr = c_filename.as_ptr();
    let filename_cstr = CStr::from_ptr(ptr);
    let filename_str = filename_cstr.to_str().unwrap();
    std::fs::read_to_string(filename_str).unwrap_or_else(|_| "read_error".to_string())
}
fn main() {
    let xml_input = "<!DOCTYPE data [<!ENTITY xxe SYSTEM \"vulnerable.txt\">]><data>&xxe;</data>";
    let parser = Parser::new();
    match parser.parse(xml_input) {
        Ok(output) => println!("Parsed output:\n{}", output),
        Err(err) => eprintln!("Parsing error: {}", err),
    }
}
