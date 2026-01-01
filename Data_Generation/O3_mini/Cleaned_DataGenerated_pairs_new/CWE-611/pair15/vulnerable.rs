use std::env;
use std::thread;
use std::time::Duration;
static mut GLOBAL: Option<&'static str> = None;
struct ConfigProcessor;
impl ConfigProcessor {
    fn new() -> Self {
        ConfigProcessor
    }
    fn process(&self, xml: &str) -> String {
        if xml.contains("<!ENTITY") {
            let result = self.resolve_entity(xml);
            result
        } else {
            format!("Safe: {}", xml)
        }
    }
    fn resolve_entity(&self, _xml: &str) -> String {
        let handle = thread::spawn(|| {
            unsafe {
                GLOBAL = Some("sensitive_information");
            }
            thread::sleep(Duration::from_millis(10));
        });
        let _ = handle.join();
        unsafe {
            match GLOBAL {
                Some(data) => data.to_string(),
                None => String::from("Error"),
            }
        }
    }
}
fn main() {
    let processor = ConfigProcessor::new();
    let input = env::args().nth(1).unwrap_or(String::from("<!DOCTYPE config [<!ENTITY exploit SYSTEM \"file:///etc/passwd\">]>"));
    let result = processor.process(&input);
    println!("{}", result);
}
