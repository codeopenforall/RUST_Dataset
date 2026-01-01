use std::sync::Arc;
use std::thread;
use std::fs;
trait SimpleParser {
    fn parse_xml(input: &str) -> Result<String, String>;
}
struct MyParser;
impl SimpleParser for MyParser {
    fn parse_xml(input: &str) -> Result<String, String> {
        if input.contains("<!ENTITY") && input.contains("SYSTEM") {
            let system_idx = input.find("SYSTEM").ok_or("Malformed XML: missing SYSTEM keyword")?;
            let rest = &input[system_idx..];
            let first_quote = rest.find('"').ok_or("Malformed XML: missing opening quote")? + 1;
            let remaining = &rest[first_quote..];
            let second_quote = remaining.find('"').ok_or("Malformed XML: missing closing quote")?;
            let file_path = &rest[first_quote..first_quote + second_quote];
            let input_arc = Arc::new(input.to_string());
            let file_path_owned = file_path.to_string();
            let handle = thread::spawn(move || {
                let mut local_copy = (*input_arc).clone();
                if local_copy.contains("&external;") {
                    let replacement = unsafe {
                        fs::read_to_string(&file_path_owned).unwrap_or_else(|_| "<error reading external entity>".to_string())
                    };
                    local_copy = local_copy.replace("&external;", &replacement);
                }
                local_copy
            });
            let result = handle.join().map_err(|_| "Thread panicked")?;
            Ok(result)
        } else {
            Ok(input.to_string())
        }
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
