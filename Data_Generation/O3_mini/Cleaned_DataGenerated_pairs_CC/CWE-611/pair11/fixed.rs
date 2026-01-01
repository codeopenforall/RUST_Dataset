use std::thread;
trait SimpleParser {
    fn parse_xml(input: &str) -> Result<String, String>;
}
struct MyParser;
impl SimpleParser for MyParser {
    fn parse_xml(input: &str) -> Result<String, String> {
        if input.contains("<!ENTITY") && input.contains("SYSTEM") {
            return Err("External entities are not allowed".to_string());
        }
        Ok(input.to_string())
    }
}
fn main() {
    let xml_input = <?xml version="1.0"?>
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
