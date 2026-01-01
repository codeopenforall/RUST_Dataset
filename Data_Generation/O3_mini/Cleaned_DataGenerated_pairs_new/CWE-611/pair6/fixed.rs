use std::fs;
use std::thread;
struct Document {
    content: String,
}
impl Document {
    fn new(xml: &str) -> Self {
        if xml.contains("<!ENTITY") {
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
    let xml_input = 
    <?xml version="1.0"?>
    <!DOCTYPE sample [
      <!ENTITY ext SYSTEM "external.txt">
    ]>
    <sample>&ext;</sample>
    "#;
    let handle = thread::spawn(move || {
        let doc = Document::new(xml_input);
        println!("Parsed content: {}", doc.content);
    });
    handle.join().unwrap();
}
