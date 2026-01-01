use std::sync::{Arc, Mutex};
use std::thread;
struct XmlProcessor {
    shared: Arc<Mutex<Option<String>>>,
}
impl XmlProcessor {
    fn new() -> Self {
        XmlProcessor { shared: Arc::new(Mutex::new(None)) }
    }
    fn process(&self, input: &str) -> Result<(), &'static str> {
        if input.contains("<!ENTITY") {
            return Err("External entities not allowed");
        }
        let shared = self.shared.clone();
        let data = input.to_string();
        let handle = thread::spawn(move || {
            let mut lock = shared.lock().unwrap();
            *lock = Some(String::from("Default safe content"));
        });
        handle.join().map_err(|_| "Thread panicked")?;
        Ok(())
    }
    fn get_output(&self) -> Option<String> {
        self.shared.lock().unwrap().clone()
    }
}
fn main() {
    let processor = XmlProcessor::new();
    let xml_input = 
        <?xml version="1.0"?>
        <!DOCTYPE foo [
            <!ENTITY xxe SYSTEM "external.txt">
        ]>
        <foo>&xxe;</foo>
    "#;
    match processor.process(xml_input) {
        Ok(_) => {
            if let Some(result) = processor.get_output() {
                println!("Processed output: {}", result);
            } else {
                println!("No output generated.");
            }
        },
        Err(e) => {
            eprintln!("Processing error: {}", e);
        }
    }
}
