use std::sync::{Arc, Mutex};
use std::thread;

struct Parser;

impl Parser {
    // This function processes XML input safely by disallowing external entity resolution.
    fn process(&self, xml: &str) -> Result<String, String> {
        // Immediately reject any XML containing external entity definitions.
        if xml.contains("<!ENTITY") || xml.contains("&ext;") {
            return Err("External entity resolution has been disabled".to_string());
        }
        let result = Arc::new(Mutex::new(String::new()));
        let content = xml.to_string();
        let res_clone = Arc::clone(&result);
        // Spawn a thread to process the XML in a thread-safe and safe manner.
        let handle = thread::spawn(move || {
            // Safe processing: a simple replacement example without unsafe pointer manipulation.
            let processed = content.replace("&amp;", "&");
            let mut guard = res_clone.lock().unwrap();
            *guard = processed;
        });
        handle.join().map_err(|_| "Thread panicked")?;
        let final_res = Arc::try_unwrap(result)
            .map_err(|_| "Lock still has references")?
            .into_inner()
            .map_err(|_| "Mutex error")?;
        Ok(final_res)
    }
}

fn main() {
    // Benign XML input that does not include external entities.
    let xml_input = r#"<?xml version="1.0"?>
<data>&amp;</data>"#;
    let parser = Parser;
    match parser.process(xml_input) {
        Ok(res) => println!("Parsed XML: {}", res),
        Err(err) => println!("Error: {}", err),
    }
}