//////////////////////////////////////////////
// Vulnerable code sample for CWE-611
// This implementation unsafely resolves external XML entities.
// It processes an XML input string, and when detecting an external
// entity declaration, it unsafely reads the external file using raw pointer
// conversions within an unsafe block. Concurrent access via threads is used.
//////////////////////////////////////////////

use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::thread;
use std::ffi::{CString, CStr};

struct XmlProcessor;

impl XmlProcessor {
    fn new() -> Self {
        XmlProcessor
    }

    // Unsafely resolve the external entity: reads file content based on the entity URI.
    // This function is vulnerable because it uses an unsafe conversion from a CString pointer
    // and does not properly validate the input URI before opening a file.
    unsafe fn resolve_entity(&self, entity: &str) -> String {
        // Create a C-compatible string from the entity URI
        let c_str = CString::new(entity).unwrap();
        let ptr = c_str.as_ptr(); // raw pointer obtained
        // Convert back to &str unsafely
        let path = CStr::from_ptr(ptr).to_str().unwrap();
        // Open the file without validating that the file is safe to access.
        let mut file = File::open(path).expect("Failed to open external file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read external file");
        content
    }

    // Parse the XML input.
    // If an external entity is declared, it extracts the URI and
    // unsafely resolves it (thus permitting XXE attacks).
    fn parse(&self, input: &str) -> String {
        if input.contains("<!ENTITY ext SYSTEM") {
            // Extract the file path between the first pair of quotes.
            let first_quote = input.find('"').unwrap() + 1;
            let second_quote = input[first_quote..].find('"').unwrap() + first_quote;
            let entity = &input[first_quote..second_quote];
            // Unsafe block: external file is accessed without proper checks.
            unsafe { self.resolve_entity(entity) }
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