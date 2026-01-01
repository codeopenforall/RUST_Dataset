use std::ffi::CStr;
use std::sync::{Arc, Mutex};
use std::thread;
trait XmlProcessor {
    fn process(&self, input: &str) -> String;
}
struct Parser {
    allow_entities: bool,
}
impl Parser {
    fn new() -> Self {
        Parser { allow_entities: true }
    }
    fn parse_xml(&self, input: &str) -> String {
        let mut output = input.to_string();
        if self.allow_entities && input.contains("SYSTEM") {
            let resolver = Arc::new(Mutex::new(String::new()));
            let resolver_clone = Arc::clone(&resolver);
            let handle = thread::spawn(move || {
                let resolved_value = unsafe { Self::dangerous_resolution() };
                let mut data = resolver_clone.lock().unwrap();
                *data = resolved_value;
            });
            handle.join().unwrap();
            let resolved_str = resolver.lock().unwrap().clone();
            output = output.replace("&xxe;", &resolved_str);
        }
        output
    }
    unsafe fn dangerous_resolution() -> String {
        let raw_ptr: *const u8 = b"SensitiveDataFromFile\0".as_ptr();
        let c_str = CStr::from_ptr(raw_ptr as *const i8);
        c_str.to_string_lossy().into_owned()
    }
}
impl XmlProcessor for Parser {
    fn process(&self, input: &str) -> String {
        self.parse_xml(input)
    }
}
fn main() {
    let parser = Parser::new();
    let xml_input = <!DOCTYPE data [
<!ENTITY xxe SYSTEM "file:///etc/passwd">
]>
<data>&xxe;</data>"#;
    let result = parser.process(xml_input);
    println!("{}", result);
}
