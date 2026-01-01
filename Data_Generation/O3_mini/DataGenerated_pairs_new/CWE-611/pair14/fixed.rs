//////////////////////////////////////////////
// Fixed Example for CWE-611 (XXE)
// This version mitigates the vulnerability by rejecting any XML input
// containing external entity declarations. It ensures that the parser does
// not process external entities, thereby preventing unauthorized file access.
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct XmlProcessor {
    shared: Arc<Mutex<Option<String>>>,
}

impl XmlProcessor {
    fn new() -> Self {
        XmlProcessor { shared: Arc::new(Mutex::new(None)) }
    }

    // Processes the XML input.
    // Immediately returns an error if an external entity definition is detected.
    fn process(&self, input: &str) -> Result<(), &'static str> {
        // SAFE: Reject any XML input with external entity declarations.
        if input.contains("<!ENTITY") {
            return Err("External entities not allowed");
        }
        let shared = self.shared.clone();
        let data = input.to_string();
        let handle = thread::spawn(move || {
            // Normal safe processing without external entity resolution.
            let mut lock = shared.lock().unwrap();
            *lock = Some(String::from("Default safe content"));
        });
        handle.join().map_err(|_| "Thread panicked")?;
        Ok(())
    }

    fn get_output(&self) -> Option<String> {
        self.shared.lock().unwrap().clone()
    }
}

fn main() {
    let processor = XmlProcessor::new();
    // Sample XML input that contains an external entity.
    // The fixed implementation will reject such input.
    let xml_input = r#"
        <?xml version="1.0"?>
        <!DOCTYPE foo [
            <!ENTITY xxe SYSTEM "external.txt">
        ]>
        <foo>&xxe;</foo>
    "#;
    match processor.process(xml_input) {
        Ok(_) => {
            if let Some(result) = processor.get_output() {
                println!("Processed output: {}", result);
            } else {
                println!("No output generated.");
            }
        },
        Err(e) => {
            eprintln!("Processing error: {}", e);
        }
    }
}