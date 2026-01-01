//////////////////////////////////////////////
// Vulnerable implementation susceptible to CWE-611 
// (Improper Restriction of XML External Entities)
// This code simulates an XML configuration processor that
// processes external entity declarations. In the case that the 
// input XML contains an external entity (i.e. <!ENTITY ...>),
// it spawns a thread and unsafely “resolves” the entity by reading 
// sensitive data via an unsafe mutable global variable.
// This unsanitized external entity resolution may allow an attacker 
// to coerce the program into revealing sensitive information.
//////////////////////////////////////////////

use std::env;
use std::thread;
use std::time::Duration;

static mut GLOBAL: Option<&'static str> = None;

struct ConfigProcessor;

impl ConfigProcessor {
    fn new() -> Self {
        ConfigProcessor
    }
    
    // Processes the XML configuration.
    // If an external entity is detected, it resolves it.
    fn process(&self, xml: &str) -> String {
        if xml.contains("<!ENTITY") {
            // Dangerous: unsafely resolve external entities.
            let result = self.resolve_entity(xml);
            result
        } else {
            format!("Safe: {}", xml)
        }
    }
    
    // This method simulates the resolution of an XML external entity.
    // It spawns a thread that unsafely assigns sensitive data to a global pointer.
    fn resolve_entity(&self, _xml: &str) -> String {
        let handle = thread::spawn(|| {
            // UNSAFE operation: simulate resolution of an external entity
            unsafe {
                GLOBAL = Some("sensitive_information");
            }
            thread::sleep(Duration::from_millis(10));
        });
        
        let _ = handle.join();
        
        unsafe {
            // UNSAFE dereference of a globally set pointer that may hold sensitive data.
            match GLOBAL {
                Some(data) => data.to_string(),
                None => String::from("Error"),
            }
        }
    }
}

fn main() {
    let processor = ConfigProcessor::new();
    // Use a default malicious XML input if none is provided.
    let input = env::args().nth(1).unwrap_or(String::from("<!DOCTYPE config [<!ENTITY exploit SYSTEM \"file:///etc/passwd\">]>"));
    let result = processor.process(&input);
    println!("{}", result);
}