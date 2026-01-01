//////////////////////////////////////////////////////////
// Secure implementation: XML parser with disabled external
// entity resolution. The parser now explicitly rejects XML that
// declares external entities. This prevents XXE attacks.
//////////////////////////////////////////////////////////

use std::thread;

trait SimpleParser {
    fn parse_xml(input: &str) -> Result<String, String>;
}

struct MyParser;

impl SimpleParser for MyParser {
    fn parse_xml(input: &str) -> Result<String, String> {
        // Check for any declaration of an external entity and reject it.
        if input.contains("<!ENTITY") && input.contains("SYSTEM") {
            return Err("External entities are not allowed".to_string());
        }
        // No external entities found; proceed safely.
        Ok(input.to_string())
    }
}

fn main() {
    // Example input that would ordinarily include an external entity.
    // Now, the parser rejects such input.
    let xml_input = r#"<?xml version="1.0"?>
<!DOCTYPE root [
<!ENTITY external SYSTEM "test.txt">
]>
<root>
    &external;
</root>"#;
    match MyParser::parse_xml(xml_input) {
        Ok(parsed) => println!("Parsed XML:\n{}", parsed),
        Err(err) => eprintln!("Error: {}", err),
    }
}