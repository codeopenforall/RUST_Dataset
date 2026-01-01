use std::fs;
struct XMLDoc {
    content: String,
}
impl XMLDoc {
    fn new(xml: &str) -> Self {
        XMLDoc {
            content: xml.to_owned(),
        }
    }
    fn process(&self) -> Result<String, String> {
        if self.content.contains("<!DOCTYPE") && self.content.contains("SYSTEM") {
            return Err("External entity resolution disabled".to_owned());
        }
        Ok("XML parsed successfully without external entity.".to_owned())
    }
}
fn main() {
    let xml_input = <?xml version="1.0"?>
<!DOCTYPE foo [
<!ENTITY xxe SYSTEM "vulnerable_resource.txt">
]>
<root>&xxe;</root>"#;
    let document = XMLDoc::new(xml_input);
    match document.process() {
        Ok(data) => println!("Result: {}", data),
        Err(err) => println!("Error: {}", err),
    }
}
