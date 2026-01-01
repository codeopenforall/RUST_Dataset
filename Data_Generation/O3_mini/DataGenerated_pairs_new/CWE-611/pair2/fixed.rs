//////////////////////////////////////////////////////////////////////
// Fixed Implementation - External Entities Are Disabled
//////////////////////////////////////////////////////////////////////

use std::ffi::CStr;
use std::sync::{Arc, Mutex};
use std::thread;

// Define the same XML processing contract.
trait XmlProcessor {
    fn process(&self, input: &str) -> String;
}

struct Parser {
    // Flag now controls external entity resolution.
    allow_entities: bool,
}

impl Parser {
    fn new() -> Self {
        // Disable external entity resolution to mitigate CWE-611.
        Parser { allow_entities: false }
    }

    fn parse_xml(&self, input: &str) -> String {
        // With external entity resolution disabled, input is returned unmodified.
        // Even if the input contains external entity declarations, they are not processed.
        if self.allow_entities && input.contains("SYSTEM") {
            let mut output = input.to_string();
            let resolver = Arc::new(Mutex::new(String::new()));
            let resolver_clone = Arc::clone(&resolver);

            let handle = thread::spawn(move || {
                // Although the unsafe function remains defined, it won't be invoked.
                let resolved_value = unsafe { Self::dangerous_resolution() };
                let mut data = resolver_clone.lock().unwrap();
                *data = resolved_value;
            });

            handle.join().unwrap();
            let resolved_str = resolver.lock().unwrap().clone();
            output = output.replace("&xxe;", &resolved_str);
            output
        } else {
            // Simply return the original input with external entities intact.
            input.to_string()
        }
    }

    // The dangerous resolution function remains but is never reached.
    unsafe fn dangerous_resolution() -> String {
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

// The main function demonstrates the secure application behavior.
fn main() {
    let parser = Parser::new();
    let xml_input = r#"<!DOCTYPE data [
<!ENTITY xxe SYSTEM "file:///etc/passwd">
]>
<data>&xxe;</data>"#;
    let result = parser.process(xml_input);
    println!("{}", result);
}