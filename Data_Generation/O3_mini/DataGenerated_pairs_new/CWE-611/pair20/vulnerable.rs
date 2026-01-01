////////////////////////////////////////////////////////////////
// Vulnerable module: This code parses an XML string and, if it
// detects an external entity declaration (e.g. <!ENTITY ... SYSTEM "...">),
// it erroneously attempts to resolve it by unsafely reading the referenced file.
// It uses an unsafe block and pointer manipulation to simulate external entity
// resolution – a pattern seen in real-world issues where XML parsers fail to disable
// external entity expansion (CWE-611). This behavior could lead to unintended file
// disclosure if an attacker can control XML inputs.
////////////////////////////////////////////////////////////////
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

struct Parser {
    xml_data: String,
}

impl Parser {
    fn new(xml_data: String) -> Parser {
        Parser { xml_data }
    }

    fn parse(&self) -> Result<String, String> {
        // Naively detect external entity declarations
        if self.xml_data.contains("<!ENTITY") && self.xml_data.contains("SYSTEM") {
            // Problematic: resolves external entities unsafely
            let system_uri = self.extract_system_uri();
            if let Some(uri) = system_uri {
                // Unsafe call that performs external resource access based on XML input
                let result = unsafe { self.resolve_entity(uri) };
                return result;
            }
            return Err("Invalid ENTITY declaration".to_string());
        }
        Ok("XML parsed without external entities".to_string())
    }

    fn extract_system_uri(&self) -> Option<&str> {
        // Extremely naive extraction: finds "SYSTEM \"" then extracts the URI until the next quote.
        if let Some(start) = self.xml_data.find("SYSTEM \"") {
            let offset = start + "SYSTEM \"".len();
            if let Some(end) = self.xml_data[offset..].find("\"") {
                return Some(&self.xml_data[offset..offset + end]);
            }
        }
        None
    }

    // This function unsafely resolves an external entity.
    // It uses pointer operations to retrieve a file path and then reads the file.
    unsafe fn resolve_entity(&self, uri: &str) -> Result<String, String> {
        if uri.starts_with("file://") {
            let path = &uri[7..];
            // Simulate unsafe pointer usage with a C-style string conversion.
            let c_string = std::ffi::CString::new(path).unwrap();
            let ptr = c_string.as_ptr();
            let slice = std::slice::from_raw_parts(ptr as *const u8, c_string.to_bytes().len());
            // Attempt to open the file indicated by the external entity.
            let mut file = File::open(std::str::from_utf8(slice).unwrap())
                .map_err(|_| "Failed to open file".to_string())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|_| "Failed to read file".to_string())?;
            Ok(contents)
        } else {
            Err("Unsupported URI type".to_string())
        }
    }
}

fn process(xml: String, result: Arc<Mutex<Option<String>>>) {
    let parser = Parser::new(xml);
    let res = parser.parse();
    let mut guard = result.lock().unwrap();
    *guard = Some(res.unwrap_or_else(|_| "Error".to_string()));
}

pub fn main() {
    // Example XML containing an external entity that points to a local file.
    let xml_input = r#"<?xml version="1.0"?>
<!DOCTYPE data [
    <!ENTITY foo SYSTEM "file://./test_input.txt">
]>
<data>&foo;</data>"#;

    let result_data = Arc::new(Mutex::new(None));
    let xml_copy = xml_input.to_string();
    let res_copy = Arc::clone(&result_data);

    let handle = thread::spawn(move || {
        process(xml_copy, res_copy);
    });
    handle.join().unwrap();
    
    let output = result_data.lock().unwrap().clone().unwrap();
    println!("Parsed content: {}", output);
}