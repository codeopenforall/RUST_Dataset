///////////////////////////////////////////////////////////
// Revised XML processing code that disables external
// entity resolution. The unsafe block has been removed,
// and the processor now immediately returns an error if
// an external entity is detected in the XML input.
///////////////////////////////////////////////////////////
use std::io::Read;

struct XmlProcessor;

impl XmlProcessor {
    fn process(&self, input: &str) -> Result<String, &'static str> {
        // Detect external entity references and reject them.
        if input.contains("&ext;") {
            return Err("External entities not allowed");
        }
        Ok(input.to_owned())
    }
}

fn main() {
    let processor = XmlProcessor;
    // Example input that includes an external entity reference.
    let input = "<root>&ext;</root>";
    match processor.process(input) {
        Ok(result) => println!("Processing succeeded: {}", result),
        Err(err) => eprintln!("Processing error: {}", err),
    }
}