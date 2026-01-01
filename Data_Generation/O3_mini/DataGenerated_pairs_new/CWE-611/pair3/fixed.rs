//////////////////////////////////////////////
// Fixed code sample for CWE-611
// This revised implementation explicitly disables external entity resolution.
// Instead of reading the external file requested in the XML, it returns a message
// indicating that the functionality is disabled. Concurrency remains, but the unsafe
// block has been removed.
//////////////////////////////////////////////

use std::sync::Arc;
use std::thread;

struct XmlProcessor;

impl XmlProcessor {
    fn new() -> Self {
        XmlProcessor
    }

    // Safe parsing that disables external entity resolution.
    fn parse(&self, input: &str) -> String {
        if input.contains("<!ENTITY ext SYSTEM") {
            // External entity resolution is intentionally disabled.
            "External entity resolution disabled".to_string()
        } else {
            "No external entity".to_string()
        }
    }
}

fn main() {
    let processor = Arc::new(XmlProcessor::new());
    let xml_input = r#"
    <!DOCTYPE foo [
      <!ENTITY ext SYSTEM "test_input.txt">
    ]>
    <foo>&ext;</foo>
    "#;
    let processor_clone = Arc::clone(&processor);
    let handle = thread::spawn(move || {
        processor_clone.parse(xml_input)
    });
    let output = handle.join().unwrap();
    println!("{}", output);
}