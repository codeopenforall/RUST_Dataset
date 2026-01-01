use std::sync::{Arc, Mutex};
use std::thread;
struct Parser {
    xml_data: String,
}
impl Parser {
    fn new(xml_data: String) -> Parser {
        Parser { xml_data }
    }
    fn parse(&self) -> Result<String, String> {
        if self.xml_data.contains("<!ENTITY") && self.xml_data.contains("SYSTEM") {
            return Err("External entity processing is disabled".to_string());
        }
        Ok("XML parsed safely; external entities not allowed".to_string())
    }
}
fn process(xml: String, result: Arc<Mutex<Option<String>>>) {
    let parser = Parser::new(xml);
    let res = parser.parse();
    let mut guard = result.lock().unwrap();
    *guard = Some(res.unwrap_or_else(|_| "Error".to_string()));
}
pub fn main() {
    let xml_input = <?xml version="1.0"?>
<!DOCTYPE data [
    <!ENTITY foo SYSTEM "file://./test_input.txt">
]>
<data>&foo;</data>"#;
    let result_data = Arc::new(Mutex::new(None));
    let xml_copy = xml_input.to_string();
    let res_copy = Arc::clone(&result_data);
    let handle = thread::spawn(move || {
        process(xml_copy, res_copy);
    });
    handle.join().unwrap();
    let output = result_data.lock().unwrap().clone().unwrap();
    println!("Parsed content: {}", output);
}
