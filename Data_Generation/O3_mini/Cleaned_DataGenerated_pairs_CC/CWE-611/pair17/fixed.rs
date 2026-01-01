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
    pub fn parse(&self, input: &str) -> Result<String, String> {
        if input.contains("&xxe;") || input.contains("<!ENTITY") {
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
