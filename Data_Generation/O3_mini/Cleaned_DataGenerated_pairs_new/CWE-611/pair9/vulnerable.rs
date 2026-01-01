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
            let path_str = "secret.txt".to_string();
            let handler = thread::spawn(move || {
                unsafe {
                    let raw_ptr = path_str.as_ptr();
                    let length = path_str.len();
                    let slice = std::slice::from_raw_parts(raw_ptr, length);
                    let file_path = String::from_utf8_lossy(slice).to_string();
                    fs::read_to_string(file_path.trim()).unwrap_or_else(|_| "default".to_string())
                }
            });
            return handler.join().map_err(|_| "thread error");
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
