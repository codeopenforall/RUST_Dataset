//////////////////////////////////////////////
// Vulnerable Example for CWE-611 (XXE)
// This version unsafely resolves external XML entities, which can
// allow an attacker to trick the program into reading external files.
// It uses an unsafe block with raw pointer conversion and concurrency
// via thread spawning.
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs;

struct XmlProcessor {
    shared: Arc<Mutex<Option<String>>>,
}

impl XmlProcessor {
    fn new() -> Self {
        XmlProcessor { shared: Arc::new(Mutex::new(None)) }
    }

    // Processes the XML input.
    // If an external entity definition is found, it unsafely reads and includes
    // content from an external file. This is the core vulnerability.
    fn process(&self, input: &str) -> Result<(), &'static str> {
        let shared = self.shared.clone();
        let data = input.to_string();
        let handle = thread::spawn(move || {
            if data.contains("<!ENTITY") {
                // UNSAFE: External entity resolution without proper restrictions.
                unsafe {
                    // Attempt to read external file; an attacker may control this.
                    let content = fs::read_to_string("external.txt").unwrap_or_else(|_| String::new());
                    // INSECURE: Using raw pointer to convert file content into a string slice.
                    let ptr = content.as_ptr();
                    let len = content.len();
                    let slice = std::slice::from_raw_parts(ptr, len);
                    let result = String::from_utf8_lossy(slice).into_owned();
                    
                    // Simulate storing the result in a shared buffer.
                    let mut lock = shared.lock().unwrap();
                    *lock = Some(result);
                }
            } else {
                let mut lock = shared.lock().unwrap();
                *lock = Some(String::from("Default content"));
            }
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
    let xml_input = r#"
        <?xml version="1.0"?>
        <!DOCTYPE foo [
            <!ENTITY xxe SYSTEM "external.txt">
        ]>
        <foo>&xxe;</foo>
    "#;
    if let Err(e) = processor.process(xml_input) {
        eprintln!("Processing error: {}", e);
        return;
    }
    if let Some(result) = processor.get_output() {
        println!("Processed output: {}", result);
    } else {
        println!("No output generated.");
    }
}