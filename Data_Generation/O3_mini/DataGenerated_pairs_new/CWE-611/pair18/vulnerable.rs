////////////////////////////////////////////
// Vulnerable Code Sample for CWE-611 (XXE)
// This program parses an XML string and unsafely resolves external entities.
// It uses unsafe blocks and concurrency constructs, which, along with
// improperly enabled external entity resolution, can lead to unintended file
// disclosure.
////////////////////////////////////////////

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct XmlEngine;

impl XmlEngine {
    // Public function used by tests.
    pub fn process_xml(input: &str) -> Result<String, Box<dyn Error>> {
        // Check for presence of a DOCTYPE with an external entity.
        if input.contains("<!ENTITY ext SYSTEM") {
            // Spawn a thread to simulate concurrent resolution of an external entity.
            let shared_secret = Arc::new(Mutex::new(String::new()));
            let secret_handle = Arc::clone(&shared_secret);
            let handle = thread::spawn(move || {
                // Unsafe block simulating an unsafe external entity resolution.
                unsafe {
                    // Simulate reading a file "secret.txt" by directly accessing a raw pointer.
                    // In a real-world scenario this could cause information disclosure.
                    let secret_data = "SECRET_DATA";
                    let ptr = secret_data.as_ptr();
                    let len = secret_data.len();
                    let slice = std::slice::from_raw_parts(ptr, len);
                    let resolved = std::str::from_utf8(slice).unwrap().to_owned();
                    let mut guard = secret_handle.lock().unwrap();
                    *guard = resolved;
                }
            });
            handle.join().unwrap();
            let resolved_value = shared_secret.lock().unwrap().clone();
            // Replace entity occurrence with the resolved string.
            let output = input.replace("&ext;", &resolved_value);
            Ok(output)
        } else {
            Ok(input.to_owned())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Example XML input that contains an external entity.
    let xml_data = "<!DOCTYPE test [<!ENTITY ext SYSTEM \"secret.txt\">]><test>&ext;</test>";
    let result = XmlEngine::process_xml(xml_data)?;
    // In the vulnerable build, the external entity is resolved to "SECRET_DATA".
    println!("Resulting XML: {}", result);
    Ok(())
}