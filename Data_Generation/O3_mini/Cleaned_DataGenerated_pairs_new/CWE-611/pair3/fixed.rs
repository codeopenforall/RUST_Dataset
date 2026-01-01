use std::sync::Arc;
use std::thread;
struct XmlProcessor;
impl XmlProcessor {
    fn new() -> Self {
        XmlProcessor
    }
    fn parse(&self, input: &str) -> String {
        if input.contains("<!ENTITY ext SYSTEM") {
            "External entity resolution disabled".to_string()
        } else {
            "No external entity".to_string()
        }
    }
}
fn main() {
    let processor = Arc::new(XmlProcessor::new());
    let xml_input = 
    <!DOCTYPE foo [
      <!ENTITY ext SYSTEM "test_input.txt">
    ]>
    <foo>&ext;</foo>
    "#;
    let processor_clone = Arc::clone(&processor);
    let handle = thread::spawn(move || {
        processor_clone.parse(xml_input)
    });
    let output = handle.join().unwrap();
    println!("{}", output);
}
