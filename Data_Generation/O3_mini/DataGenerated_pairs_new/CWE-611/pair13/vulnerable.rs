///////////////////////////////////////////////////////////////////////////////////////////////////////
// WARNING: This code contains a security flaw related to improper XML external entity resolution.
// It unsafely processes XML input containing external entity declarations without proper restrictions.
// This code is for demonstration purposes only.
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CStr;
use std::os::raw::c_char;

struct XMLProcessor;

impl XMLProcessor {
    fn new() -> Self {
        XMLProcessor
    }

    // This method unsafely processes XML input. It does not restrict XML external entities.
    fn process(&self, input: &str) -> Result<String, &'static str> {
        // If the input contains an entity declaration, it proceeds unsafely.
        if input.contains("<!ENTITY") {
            unsafe {
                // UNSAFE: Simulate external entity resolution by returning a pointer to static secret data.
                let secret: *const c_char = b"SECRET\0".as_ptr() as *const c_char;
                // The use of CStr assumes the pointer is valid and null-terminated.
                let sec_str = CStr::from_ptr(secret).to_str().unwrap();
                return Ok(sec_str.to_string());
            }
        }
        // Otherwise, perform a naive XML "parsing" simulation.
        Ok("Parsed Content".to_string())
    }
}

fn main() {
    // Use Arc and Mutex for simulated concurrent processing.
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