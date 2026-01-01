//////////////////////////////
// Begin Vulnerable Code
//////////////////////////////
use std::fs;
use std::ptr;
use std::thread;

struct Document {
    content: String,
}

impl Document {
    fn new(xml: &str) -> Self {
        // This parser implementation naively and unsafely resolves external entities.
        // It looks for an external entity declaration and then tries to read the file it names.
        if xml.contains("<!ENTITY") {
            // For simplicity, assume a declaration of the form: <!ENTITY ext SYSTEM "external.txt">
            if let Some(system_pos) = xml.find("SYSTEM") {
                let rest = &xml[system_pos + 6..];
                if let Some(start_quote) = rest.find("\"") {
                    let rest = &rest[start_quote + 1..];
                    if let Some(end_quote) = rest.find("\"") {
                        let file_path = &rest[..end_quote];
                        // Unsafe block used to emulate careless pointer manipulation.
                        unsafe {
                            let file_ptr = file_path.as_ptr();
                            // Create a slice from the raw pointer without proper bounds checking.
                            let file_slice = std::slice::from_raw_parts(file_ptr, file_path.len());
                            // Construct a &str without checking UTF-8 safety.
                            let file_str = std::str::from_utf8_unchecked(file_slice);
                            // Simulated external file read. No restrictions are applied!
                            if let Ok(contents) = fs::read_to_string(file_str) {
                                return Document { content: contents };
                            }
                        }
                    }
                }
            }
        }
        Document {
            content: xml.to_string(),
        }
    }
}

fn main() {
    let xml_input = r#"
    <?xml version="1.0"?>
    <!DOCTYPE sample [
      <!ENTITY ext SYSTEM "external.txt">
    ]>
    <sample>&ext;</sample>
    "#;
    // Spawn a thread to process the XML concurrently.
    let handle = thread::spawn(move || {
        let doc = Document::new(xml_input);
        println!("Parsed content: {}", doc.content);
    });
    handle.join().unwrap();
}
//////////////////////////////
// End Vulnerable Code
//////////////////////////////