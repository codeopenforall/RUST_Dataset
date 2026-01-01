use std::sync::{Arc, Mutex};

struct Processor {
    flag: Arc<Mutex<bool>>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            flag: Arc::new(Mutex::new(true)),
        }
    }

    // This secure method processes XML input by explicitly disallowing external entity resolution.
    // If a DOCTYPE declaration is detected, the input is rejected instead of performing any unsafe operations.
    fn process(&self, input: &str) -> String {
        if input.contains("<!DOCTYPE") {
            // Prevent any external entity processing by rejecting the input altogether.
            return "Error: External entities not allowed".to_string();
        }
        input.to_string()
    }
}

fn main() {
    // The input XML contains an external entity declaration.
    // In this secure version, the presence of a DOCTYPE triggers a rejection.
    let xml = r#"<?xml version="1.0"?>
<!DOCTYPE data [
  <!ENTITY xxe SYSTEM "external.txt">
]>
<data>&xxe;</data>
"#;
    let proc_instance = Processor::new();
    let output = proc_instance.process(xml);
    println!("Processed output: {}", output);
}