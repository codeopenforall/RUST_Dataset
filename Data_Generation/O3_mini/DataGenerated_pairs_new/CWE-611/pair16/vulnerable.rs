/* 
   This code simulates an XML parser that is vulnerable to XXE (CWE-611) by allowing external entity resolution.
   It uses an unsafe block to manually resolve an external entity reference in the XML input, taking the file path
   from the XML declaration and reading the file contents. It also demonstrates concurrency using threads.
*/

use std::fs;
use std::ptr;
use std::thread;
use std::sync::Arc;

struct Parser;

impl Parser {
    pub fn process(&self, xml: &str) -> Result<String, String> {
        // Check if XML contains a DOCTYPE with a SYSTEM external entity
        if xml.contains("<!DOCTYPE") && xml.contains("SYSTEM") {
            // Extract the file path from the SYSTEM declaration (naive parsing).
            let start = xml.find("SYSTEM").ok_or("No SYSTEM found")?;
            // Assume the file path is enclosed in quotes.
            let quote_index = xml[start..].find("\"").ok_or("No quote found")? + start;
            let end_quote_index = xml[quote_index + 1..]
                .find("\"")
                .ok_or("No ending quote")? + quote_index + 1;
            let file_spec = &xml[quote_index + 1..end_quote_index];
            
            // UNSAFE block: directly working with raw pointers and resolving the file name.
            unsafe {
                // Obtain raw pointer and length from the file_spec string slice.
                let raw_ptr = file_spec.as_ptr();
                let raw_len = file_spec.len();
                // Reconstruct a slice from raw parts
                let check_slice = std::slice::from_raw_parts(raw_ptr, raw_len);
                let file_path = std::str::from_utf8(check_slice).map_err(|_| "Invalid UTF8")?;
                // Vulnerability: The file path is directly used to read a file, enabling XXE attacks.
                let content = fs::read_to_string(file_path).map_err(|_| "File read error")?;
                return Ok(content);
            }
        }
        Ok(xml.to_string())
    }
}

fn main() {
    let parser = Parser;
    // Vulnerable XML input containing an external entity reference.
    let xml_input = "<!DOCTYPE test [ <!ENTITY file SYSTEM \"test_input.txt\"> ]><test>&file;</test>";
    let shared = Arc::new(parser);
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        shared_clone.process(xml_input)
    });

    match handle.join() {
        Ok(result) => println!("Processed Result: {:?}", result),
        Err(_) => println!("Thread error during processing"),
    }
}