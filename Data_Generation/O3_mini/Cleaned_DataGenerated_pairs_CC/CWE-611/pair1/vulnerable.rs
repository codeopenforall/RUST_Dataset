use std::sync::{Arc, Mutex};
use std::thread;
use std::fs;
use std::ptr;
use std::ffi::CString;
struct XmlProcessor {
    allow_external: bool,
}
impl XmlProcessor {
    fn new() -> Self {
        Self { allow_external: true }
    }
    fn process(&self, input: &str) -> Result<String, String> {
        if input.contains("<!ENTITY") && self.allow_external {
            let start = input.find("SYSTEM").ok_or("No SYSTEM found")?;
            let first_quote = input[start..].find("\"").ok_or("No starting quote")? + start + 1;
            let second_quote = input[first_quote..].find("\"").ok_or("No ending quote")? + first_quote;
            let sys_id = &input[first_quote..second_quote];
            unsafe {
                let cstr = CString::new(sys_id).map_err(|_| "CString error")?;
                let ptr_raw = cstr.as_ptr();
                let cloned = CString::from_raw(ptr::read(&ptr_raw) as *mut i8);
                let file_content = fs::read_to_string(cloned.to_str().unwrap())
                    .map_err(|_| "Failed to read external file")?;
                std::mem::forget(cloned);
                Ok(file_content)
            }
        } else {
            Ok(input.to_string())
        }
    }
    fn run(&self, xml_content: &str) {
        let data = Arc::new(Mutex::new(xml_content.to_string()));
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let xml = data_clone.lock().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(10));
            xml.clone()
        });
        let xml = handle.join().unwrap();
        match self.process(&xml) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
fn main() {
    let xml_data = <!DOCTYPE data [
        <!ENTITY ext SYSTEM "test.txt">
    ]>
    <data>&ext;</data>"#;
    let processor = XmlProcessor::new();
    processor.run(xml_data);
}
