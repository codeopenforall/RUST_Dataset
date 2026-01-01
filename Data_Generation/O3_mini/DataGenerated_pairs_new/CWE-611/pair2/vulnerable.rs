//////////////////////////////////////////////////////////////////////
// Vulnerable Implementation - Improper Restriction of XML External Entities
//////////////////////////////////////////////////////////////////////

use std::ffi::CStr;
use std::sync::{Arc, Mutex};
use std::thread;

// Define a common contract for XML processing.
trait XmlProcessor {
    fn process(&self, input: &str) -> String;
}

struct Parser {
    // This flag enables external entity resolution.
    allow_entities: bool,
}

impl Parser {
    fn new() -> Self {
        // By default, external entity resolution is enabled.
        Parser { allow_entities: true }
    }

    fn parse_xml(&self, input: &str) -> String {
        let mut output = input.to_string();

        // Look for an external entity directive.
        if self.allow_entities && input.contains("SYSTEM") {
            // Simulate external entity resolution by launching a thread.
            let resolver = Arc::new(Mutex::new(String::new()));
            let resolver_clone = Arc::clone(&resolver);

            let handle = thread::spawn(move || {
                // UNSAFE: This block unsafely resolves an external entity
                // by reading a string from a raw pointer. In a real-world scenario,
                // this could lead to disclosure of sensitive data if the file is not trusted.
                let resolved_value = unsafe { Self::dangerous_resolution() };
                let mut data = resolver_clone.lock().unwrap();
                *data = resolved_value;
            });

            handle.join().unwrap();
            let resolved_str = resolver.lock().unwrap().clone();
            // Replace the entity reference with the externally provided data.
            output = output.replace("&xxe;", &resolved_str);
        }
        output
    }

    // UNSAFE function that simulates dangerous external entity resolution.
    unsafe fn dangerous_resolution() -> String {
        // The raw pointer is obtained from a constant byte string.
        // In a realistic vulnerable implementation, this might trigger an external file read.
        let raw_ptr: *const u8 = b"SensitiveDataFromFile\0".as_ptr();
        let c_str = CStr::from_ptr(raw_ptr as *const i8);
        c_str.to_string_lossy().into_owned()
    }
}

impl XmlProcessor for Parser {
    fn process(&self, input: &str) -> String {
        self.parse_xml(input)
    }
}

// The main function simulates an application that processes XML input.
fn main() {
    let parser = Parser::new();
    // Example XML input containing an external entity definition.
    let xml_input = r#"<!DOCTYPE data [
<!ENTITY xxe SYSTEM "file:///etc/passwd">
]>
<data>&xxe;</data>"#;
    let result = parser.process(xml_input);
    println!("{}", result);
}