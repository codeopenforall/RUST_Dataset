//////////////////////
// Secure Demo
//////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs;

struct XmlProcessor {
    // External entity resolution is now disabled.
    allow_external: bool,
}

impl XmlProcessor {
    fn new() -> Self {
        Self { allow_external: false }
    }
    
    fn process(&self, input: &str) -> Result<String, String> {
        // If an external entity is detected, refuse to process it.
        if input.contains("<!ENTITY") {
            return Err("External entity resolution is disabled".to_string());
        }
        Ok(input.to_string())
    }
    
    fn run(&self, xml_content: &str) {
        // Use Arc and Mutex to simulate concurrent processing.
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
    // XML with an external entity; secure processing disallows retrieving external content.
    let xml_data = r#"<!DOCTYPE data [
        <!ENTITY ext SYSTEM "test.txt">
    ]>
    <data>&ext;</data>"#;
    
    let processor = XmlProcessor::new();
    processor.run(xml_data);
}