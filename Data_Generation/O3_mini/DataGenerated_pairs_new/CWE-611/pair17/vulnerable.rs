////////////////////////////////////////////////////////////////////////////////
// Vulnerable Implementation: Improper Restriction of XML External Entities (CWE-611)
// This implementation uses unsafe blocks and concurrent threads to read external files
// based on XML external entity declarations. The parser blindly allows the inclusion
// of external content without proper validation or restrictions, making it susceptible
// to XXE attacks.
////////////////////////////////////////////////////////////////////////////////
use std::ffi::{CString, CStr};
use std::thread;
use std::sync::{Arc, Mutex};

pub struct Parser {
    // using an Arc and Mutex to simulate shared state in a concurrent parser
    state: Arc<Mutex<String>>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            state: Arc::new(Mutex::new(String::new())),
        }
    }

    // Processes XML input. If an external entity "&xxe;" is detected in the XML,
    // an unsafe external file read is triggered concurrently.
    pub fn parse(&self, input: &str) -> Result<String, String> {
        let mut result = input.to_string();

        // Check for the external entity marker in the XML
        if input.contains("&xxe;") {
            // In a realistic parser, the filename would be extracted from the DOCTYPE.
            // For this simulation, we use a fixed filename.
            let filename = "vulnerable.txt";

            // Spawn a thread that performs an unsafe external file read.
            let state_clone = Arc::clone(&self.state);
            let handle = thread::spawn(move || {
                // Unsafe block: converts the filename to a C string and reads the file.
                unsafe {
                    let file_data = unsafe_fetch(filename);
                    // Update shared state with file data.
                    if let Ok(mut data) = state_clone.lock() {
                        *data = file_data;
                    }
                }
            });

            // Wait for thread completion; in real scenarios, concurrency mismanagement can occur.
            handle.join().map_err(|_| "Thread panicked".to_string())?;

            // Retrieve the file content from shared state and replace the entity.
            let file_content = self.state.lock().map_err(|_| "Lock error".to_string())?.clone();
            result = result.replace("&xxe;", &file_content);
        }
        Ok(result)
    }
}

// Unsafe function that converts a Rust string to a C-style string and then reads
// file content using standard library functions without validating the file path.
unsafe fn unsafe_fetch(file: &str) -> String {
    let c_filename = CString::new(file).unwrap();
    // Simulate unsafe pointer conversion and dereference.
    let ptr = c_filename.as_ptr();
    let filename_cstr = CStr::from_ptr(ptr);
    let filename_str = filename_cstr.to_str().unwrap();
    // Directly read the file; no restrictions are applied.
    std::fs::read_to_string(filename_str).unwrap_or_else(|_| "read_error".to_string())
}

fn main() {
    // Example XML input containing an external entity declaration.
    let xml_input = "<!DOCTYPE data [<!ENTITY xxe SYSTEM \"vulnerable.txt\">]><data>&xxe;</data>";
    let parser = Parser::new();

    match parser.parse(xml_input) {
        Ok(output) => println!("Parsed output:\n{}", output),
        Err(err) => eprintln!("Parsing error: {}", err),
    }
}