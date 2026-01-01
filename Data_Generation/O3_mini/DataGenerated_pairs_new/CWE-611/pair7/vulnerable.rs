/////////////////////////////////////////////////////////////
// Vulnerable Example: XXE with Unsafe External Entity Resolution
/////////////////////////////////////////////////////////////
use std::thread;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    // This function parses XML and resolves external entities.
    // It unsafely extracts the file path from the XML and spawns a thread
    // to read the file contents, allowing XXE vulnerability.
    pub fn parse(&self, xml: &str) -> Result<String, &'static str> {
        if xml.contains("SYSTEM") {
            // Locate the SYSTEM keyword.
            let sys_idx = xml.find("SYSTEM").ok_or("Missing SYSTEM keyword")?;
            let slice = &xml[sys_idx..];
            // Find the start of the file path by searching for the first double quote.
            let quote_start = slice.find("\"").ok_or("Missing quote")?;
            // UNSAFE: Create a slice pointer to the file path using raw pointer arithmetic.
            let remaining = unsafe {
                std::slice::from_raw_parts(slice.as_ptr().add(quote_start + 1), slice.len() - quote_start - 1)
            };
            let rem_str = std::str::from_utf8(remaining).map_err(|_| "Invalid UTF-8 in file path")?;
            if let Some(quote_end) = rem_str.find("\"") {
                let file_path = &rem_str[..quote_end];
                // Spawn a thread to perform file reading concurrently.
                let handle = thread::spawn(move || {
                    std::fs::read_to_string(file_path)
                });
                let content = handle.join().map_err(|_| "Thread join error")?
                    .map_err(|_| "Error reading external resource")?;
                return Ok(content);
            }
            return Err("Malformed SYSTEM entity declaration");
        }
        // For XML without external entity, simply return the input.
        Ok(xml.to_string())
    }
}

fn main() {
    // Example XML with an external entity referencing a local file.
    let xml = r#"<!DOCTYPE data [
       <!ENTITY ext SYSTEM "./test_entity.txt">
    ]>
    <data>&ext;</data>"#;
    
    let parser = Parser::new();
    match parser.parse(xml) {
        Ok(result) => println!("Parsed content: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}