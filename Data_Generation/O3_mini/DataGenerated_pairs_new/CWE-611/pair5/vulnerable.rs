//////////////////////////////////////////////
// Vulnerable Code for XML Processing Issue  //
// CWE-611: Improper Restriction of XML       //
// External Entities (XXE)                    //
//////////////////////////////////////////////

use std::fs;
use std::sync::{Arc, Mutex};

type ResolverFn = fn(&str) -> Result<String, String>;

struct XMLDoc {
    content: String,
    // A raw pointer to a function resolving external entities.
    resolver: Option<*const ResolverFn>,
}

impl XMLDoc {
    fn new(xml: &str) -> Self {
        XMLDoc {
            content: xml.to_owned(),
            // Assign the resolver function pointer unsafely.
            resolver: Some(resolve_external as *const ResolverFn),
        }
    }

    // Process the XML document.
    // If an external entity is detected in the DOCTYPE,
    // it extracts the system identifier and unsafely resolves it.
    fn process(&self) -> Result<String, String> {
        if self.content.contains("<!DOCTYPE") && self.content.contains("SYSTEM") {
            // Naively extract the external system identifier.
            let start = self.content.find("SYSTEM").ok_or("SYSTEM keyword not found")?;
            let quote_start = self.content[start..].find("\"").ok_or("Starting quote not found")? + start + 1;
            let quote_end = self.content[quote_start..].find("\"").ok_or("Ending quote not found")? + quote_start;
            let sys_id = &self.content[quote_start..quote_end];
            unsafe {
                let resolver_ptr = self.resolver.ok_or("No resolver function available")?;
                let resolver_fn: ResolverFn = *resolver_ptr; // UNSAFE: Dereferencing raw function pointer.
                return resolver_fn(sys_id);
            }
        }
        Ok("XML parsed successfully without external entity.".to_owned())
    }
}

// An unsafe function that simulates dangerous memory operations
// while reading an external file.
unsafe fn dangerous_read(path: &str) -> Result<String, String> {
    // Read the file content (could be external resource controlled by an attacker).
    let raw_data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    // Simulate an unsafe unchecked memory operation.
    let ptr = raw_data.as_ptr();
    let len = raw_data.len();
    let slice = std::slice::from_raw_parts(ptr, len);
    let result = String::from_utf8(slice.to_vec()).map_err(|e| e.to_string())?;
    Ok(result)
}

// A resolver function that uses the unsafe dangerous_read.
fn resolve_external(path: &str) -> Result<String, String> {
    unsafe { dangerous_read(path) }
}

fn main() {
    // Malicious XML input that includes an external entity reference.
    let xml_input = r#"<?xml version="1.0"?>
<!DOCTYPE foo [
<!ENTITY xxe SYSTEM "vulnerable_resource.txt">
]>
<root>&xxe;</root>"#;
    
    let document = XMLDoc::new(xml_input);
    match document.process() {
        Ok(data) => println!("Result: {}", data),
        Err(err) => println!("Error: {}", err),
    }
}