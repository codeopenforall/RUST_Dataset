use std::fs;
use std::thread;
use std::sync::Arc;
trait XmlProcessor {
    fn process(&self, input: &str) -> Result<String, &'static str>;
}
struct Processor;
impl XmlProcessor for Processor {
    fn process(&self, input: &str) -> Result<String, &'static str> {
        if input.contains("&external;") {
            return Err("External entities are disabled");
        }
        Ok(input.to_string())
    }
}
fn main() {
    let processor = Processor;
    let xml_input = "<!DOCTYPE test [ <!ENTITY external SYSTEM \"file:///secret.txt\"> ]><test>&external;</test>";
    match processor.process(xml_input) {
        Ok(result) => println!("Output: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
