use std::sync::{Arc, Mutex};
use std::thread;
struct XMLProcessor;
impl XMLProcessor {
    fn new() -> Self {
        XMLProcessor
    }
    fn process(&self, input: &str) -> Result<String, &'static str> {
        if input.contains("<!ENTITY") {
            return Err("External entities are disabled");
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
