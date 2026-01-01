////////////////////////////////////////////////////////////////////////////////
// Corrected Implementation: XXE Mitigation for XML Parsing (CWE-611)
// This implementation disables external entity resolution altogether. It inspects the XML input
// for any external entity markers and prevents further processing if found.
////////////////////////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};

pub struct Parser {
    state: Arc<Mutex<String>>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            state: Arc::new(Mutex::new(String::new())),
        }
    }

    // Processes XML input while disallowing any external entity resolutions.
    pub fn parse(&self, input: &str) -> Result<String, String> {
        if input.contains("&xxe;") || input.contains("<!ENTITY") {
            // Instead of processing external entities, return an error message.
            Err("External entity resolution is disabled".to_string())
        } else {
            Ok(input.to_string())
        }
    }
}

fn main() {
    let xml_input = "<data>Safe content only</data>";
    let parser = Parser::new();

    match parser.parse(xml_input) {
        Ok(output) => println!("Parsed output:\n{}", output),
        Err(err) => eprintln!("Parsing error: {}", err),
    }
}