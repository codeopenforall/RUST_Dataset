use std::io::Read;
struct XmlProcessor;
impl XmlProcessor {
    fn process(&self, input: &str) -> Result<String, &'static str> {
        if input.contains("&ext;") {
            return Err("External entities not allowed");
        }
        Ok(input.to_owned())
    }
}
fn main() {
    let processor = XmlProcessor;
    let input = "<root>&ext;</root>";
    match processor.process(input) {
        Ok(result) => println!("Processing succeeded: {}", result),
        Err(err) => eprintln!("Processing error: {}", err),
    }
}
