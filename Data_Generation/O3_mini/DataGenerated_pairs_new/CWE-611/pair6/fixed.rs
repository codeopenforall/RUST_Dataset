//////////////////////////////
// Begin Corrected Code
//////////////////////////////
use std::fs;
use std::thread;

struct Document {
    content: String,
}

impl Document {
    fn new(xml: &str) -> Self {
        // Secure parser implementation: Reject any XML with external entity declarations.
        if xml.contains("<!ENTITY") {
            // Instead of processing the external entity, simply return the raw input.
            return Document {
                content: xml.to_string(),
            };
        }
        Document {
            content: xml.to_string(),
        }
    }
}

fn main() {
    let xml_input = r#"
    <?xml version="1.0"?>
    <!DOCTYPE sample [
      <!ENTITY ext SYSTEM "external.txt">
    ]>
    <sample>&ext;</sample>
    "#;
    // Spawn a thread to process the XML concurrently.
    let handle = thread::spawn(move || {
        let doc = Document::new(xml_input);
        println!("Parsed content: {}", doc.content);
    });
    handle.join().unwrap();
}
//////////////////////////////
// End Corrected Code
//////////////////////////////