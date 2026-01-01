use std::env;
struct ConfigProcessor;
impl ConfigProcessor {
    fn new() -> Self {
        ConfigProcessor
    }
    fn process(&self, xml: &str) -> String {
        if xml.contains("<!ENTITY") {
            String::from("Rejected external entity")
        } else {
            format!("Safe: {}", xml)
        }
    }
}
fn main() {
    let processor = ConfigProcessor::new();
    let input = env::args().nth(1).unwrap_or(String::from("<data>Test</data>"));
    let result = processor.process(&input);
    println!("{}", result);
}
