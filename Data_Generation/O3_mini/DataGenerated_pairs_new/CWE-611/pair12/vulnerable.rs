use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
use std::ffi::CString;

struct Parser;

impl Parser {
    // This function processes XML input and resolves external entities.
    // It improperly permits XML External Entity (XXE) resolution.
    fn process(&self, xml: &str) -> Result<String, String> {
        let result = Arc::new(Mutex::new(String::new()));
        let content = xml.to_string();
        // Unsafe block simulating improper pointer handling and concurrency.
        unsafe {
            let c_input = CString::new(content.clone()).map_err(|_| "Null byte found")?;
            let res_clone = Arc::clone(&result);
            let handle = thread::spawn(move || {
                // Obtain a raw pointer unsafely and reconstitute a CString,
                // which incorrectly transfers ownership.
                let ptr_raw = c_input.as_ptr();
                let c_str = CString::from_raw(ptr_raw as *mut i8); // Vulnerability: improper memory management.
                let data = c_str.to_str().unwrap_or("");
                let mut out = data.to_string();
                // If an external entity is declared, attempt to resolve it.
                if data.contains("<!ENTITY ext SYSTEM") {
                    if let Some(start) = data.find("\"") {
                        if let Some(end) = data[start + 1..].find("\"") {
                            let file_path = &data[start + 1..start + 1 + end];
                            // Vulnerability: unsafe file reading via external entity.
                            if let Ok(file_content) = fs::read_to_string(file_path) {
                                out = out.replace("&ext;", file_content.as_str());
                            }
                        }
                    }
                }
                let mut guard = res_clone.lock().unwrap();
                *guard = out;
                // Avoid double free by intentionally leaking the c_str.
                std::mem::forget(c_str);
            });
            handle.join().map_err(|_| "Thread panicked")?;
        }
        let final_res = Arc::try_unwrap(result)
            .map_err(|_| "Lock still has references")?
            .into_inner()
            .map_err(|_| "Mutex error")?;
        Ok(final_res)
    }
}

fn main() {
    // Example XML input with an external entity declaration.
    let xml_input = r#"<?xml version="1.0"?>
<!DOCTYPE data [ <!ENTITY ext SYSTEM "external.txt"> ]>
<data>&ext;</data>"#;
    let parser = Parser;
    match parser.process(xml_input) {
        Ok(res) => println!("Parsed XML: {}", res),
        Err(err) => println!("Error: {}", err),
    }
}