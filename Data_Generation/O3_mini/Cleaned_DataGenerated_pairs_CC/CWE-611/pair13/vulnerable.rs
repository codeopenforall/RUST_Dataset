use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CStr;
use std::os::raw::c_char;
struct XMLProcessor;
impl XMLProcessor {
    fn new() -> Self {
        XMLProcessor
    }
    fn process(&self, input: &str) -> Result<String, &'static str> {
        if input.contains("<!ENTITY") {
            unsafe {
                let secret: *const c_char = b"SECRET\0".as_ptr() as *const c_char;
                let sec_str = CStr::from_ptr(secret).to_str().unwrap();
                return Ok(sec_str.to_string());
            }
        }
        Ok("Parsed Content".to_string())
    }
}
fn main() {
    let processor = Arc::new(Mutex::new(XMLProcessor::new()));
    let xml_inputs = vec![
        "<root><child>Test</child></root>",
        "<!DOCTYPE foo [ <!ENTITY ext SYSTEM \"file:///etc/passwd\"> ]><root>&ext;</root>"
    ];
    let mut threads = vec![];
    for xml in xml_inputs {
        let proc_clone = Arc::clone(&processor);
        let xml_str = xml.to_string();
        let handle = thread::spawn(move || {
            let proc = proc_clone.lock().unwrap();
            match proc.process(&xml_str) {
                Ok(output) => println!("Output: {}", output),
                Err(err) => println!("Error: {}", err),
            }
        });
        threads.push(handle);
    }
    for t in threads {
        t.join().unwrap();
    }
}
