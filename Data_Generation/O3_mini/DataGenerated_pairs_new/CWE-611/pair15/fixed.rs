//////////////////////////////////////////////
// Fixed implementation that mitigates CWE-611 
// by explicitly disabling external entity resolution.
// The processor now validates the XML input and rejects any input 
// that attempts to resolve external entities.
//////////////////////////////////////////////

use std::env;

struct ConfigProcessor;

impl ConfigProcessor {
    fn new() -> Self {
        ConfigProcessor
    }
    
    // Processes the XML configuration.
    // Any XML containing an external entity declaration is rejected.
    fn process(&self, xml: &str) -> String {
        if xml.contains("<!ENTITY") {
            // Explicitly reject XML with external entity definitions.
            String::from("Rejected external entity")
        } else {
            format!("Safe: {}", xml)
        }
    }
}

fn main() {
    let processor = ConfigProcessor::new();
    // Default benign XML input if none provided.
    let input = env::args().nth(1).unwrap_or(String::from("<data>Test</data>"));
    let result = processor.process(&input);
    println!("{}", result);
}