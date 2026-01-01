/////////////////////////////////////////////////////////////////
// Vulnerable Implementation: Improper Restriction of XML External Entities
/////////////////////////////////////////////////////////////////
use std::ffi::CStr;
use std::fs;
use std::thread;

struct Document {
    content: String,
}

struct Processor;

impl Processor {
    fn new() -> Self {
        Processor
    }

    // Reading and processing the XML input.
    // This method does not restrict external entity resolution.
    fn parse(&self, input: &str) -> Document {
        // Check for an external entity declaration (e.g., <!ENTITY xxe SYSTEM "vulnerable.txt">)
        if input.contains("<!ENTITY") && input.contains("SYSTEM") {
            // A crude extraction of the file path within the SYSTEM declaration.
            if let Some(start) = input.find("SYSTEM \"") {
                let rest = &input[start + 8..];
                if let Some(end) = rest.find('"') {
                    let file_path = &rest[..end];
                    // Calls the unsafe external entity loader.
                    let external_content = self.load_external(file_path);
                    return Document {
                        content: external_content,
                    };
                }
            }
        }
        Document {
            content: input.to_string(),
        }
    }

    // This method unsafely attempts to resolve the external entity.
    // It uses an unsafe block to convert a Rust string into a C-style string without proper validation,
    // and then uses thread concurrency to mimic real-world async processing.
    fn load_external(&self, path: &str) -> String {
        let p = path.to_string();
        let handle = thread::spawn(move || {
            unsafe {
                // Convert the Rust string pointer to a C-string pointer.
                // Note: p is not null-terminated, so this is inherently unsound.
                let cstr = CStr::from_ptr(p.as_ptr() as *const i8);
                match cstr.to_str() {
                    Ok(valid) => {
                        // Try to read the external file. If the file contains secret data,
                        // the attacker can force our application to load and process it.
                        fs::read_to_string(valid).unwrap_or_else(|_| "Error".to_string())
                    }
                    Err(_) => "Error".to_string(),
                }
            }
        });
        handle.join().unwrap_or_else(|_| "Error".to_string())
    }
}

fn main() {
    // XML input with an external entity declaration.
    let xml_data = r#"<?xml version="1.0"?>
<!DOCTYPE foo [
    <!ENTITY xxe SYSTEM "vulnerable.txt">
]>
<foo>&xxe;</foo>"#;
    let processor = Processor::new();
    let doc = processor.parse(xml_data);
    println!("Parsed content: {}", doc.content);
}