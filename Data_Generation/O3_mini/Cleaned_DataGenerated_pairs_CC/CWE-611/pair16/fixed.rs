use std::thread;
use std::sync::Arc;
struct Parser;
impl Parser {
    pub fn process(&self, xml: &str) -> Result<String, String> {
        if xml.contains("<!DOCTYPE") && xml.contains("SYSTEM") {
            return Err("External entity resolution disabled".to_string());
        }
        Ok(xml.to_string())
    }
}
fn main() {
    let parser = Parser;
    let xml_input = "<!DOCTYPE test [ <!ENTITY file SYSTEM \"test_input.txt\"> ]><test>&file;</test>";
    let shared = Arc::new(parser);
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        shared_clone.process(xml_input)
    });
    match handle.join() {
        Ok(result) => println!("Processed Result: {:?}", result),
        Err(_) => println!("Thread error during processing"),
    }
}
