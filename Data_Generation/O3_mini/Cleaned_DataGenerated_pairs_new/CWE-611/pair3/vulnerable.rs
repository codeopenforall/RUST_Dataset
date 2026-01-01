use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::thread;
use std::ffi::{CString, CStr};
struct XmlProcessor;
impl XmlProcessor {
    fn new() -> Self {
        XmlProcessor
    }
    unsafe fn resolve_entity(&self, entity: &str) -> String {
        let c_str = CString::new(entity).unwrap();
        let ptr = c_str.as_ptr(); 
        let path = CStr::from_ptr(ptr).to_str().unwrap();
        let mut file = File::open(path).expect("Failed to open external file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read external file");
        content
    }
    fn parse(&self, input: &str) -> String {
        if input.contains("<!ENTITY ext SYSTEM") {
            let first_quote = input.find('"').unwrap() + 1;
            let second_quote = input[first_quote..].find('"').unwrap() + first_quote;
            let entity = &input[first_quote..second_quote];
            unsafe { self.resolve_entity(entity) }
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
