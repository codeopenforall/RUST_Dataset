//////////////////////
// Vulnerability Demo
//////////////////////
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
        // If an external entity is declared and external resolution is allowed,
        // the content of the external file is read.
        if input.contains("<!ENTITY") && self.allow_external {
            // Extract the system identifier from the XML.
            let start = input.find("SYSTEM").ok_or("No SYSTEM found")?;
            let first_quote = input[start..].find("\"").ok_or("No starting quote")? + start + 1;
            let second_quote = input[first_quote..].find("\"").ok_or("No ending quote")? + first_quote;
            let sys_id = &input[first_quote..second_quote];
            
            // Unsafe block to simulate direct low-level pointer usage.
            unsafe {
                // Convert system identifier into a CString.
                let cstr = CString::new(sys_id).map_err(|_| "CString error")?;
                let ptr_raw = cstr.as_ptr();
                // Perform unsafe pointer arithmetic and reconstruct a CString.
                let cloned = CString::from_raw(ptr::read(&ptr_raw) as *mut i8);
                // Read from the file whose path is specified in the entity.
                let file_content = fs::read_to_string(cloned.to_str().unwrap())
                    .map_err(|_| "Failed to read external file")?;
                // Forget cloned to simulate a memory leak typical in unsafe code misuse.
                std::mem::forget(cloned);
                Ok(file_content)
            }
        } else {
            Ok(input.to_string())
        }
    }
    
    fn run(&self, xml_content: &str) {
        // Simulate concurrent processing using Arc and Mutex.
        let data = Arc::new(Mutex::new(xml_content.to_string()));
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let xml = data_clone.lock().unwrap();
            // Simulated processing delay.
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
    // XML payload with an external entity referencing an external file.
    let xml_data = r#"<!DOCTYPE data [
        <!ENTITY ext SYSTEM "test.txt">
    ]>
    <data>&ext;</data>"#;
    
    let processor = XmlProcessor::new();
    processor.run(xml_data);
}