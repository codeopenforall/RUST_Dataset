use std::str;
struct XmlHandler {
    external: bool,
}
impl XmlHandler {
    fn new(external: bool) -> Self {
        XmlHandler { external }
    }
    fn parse(&self, xml: &str) -> Result<String, String> {
        if xml.contains("<!ENTITY") {
            return Err("External entities are not allowed".to_string());
        }
        Ok(xml.to_string())
    }
}
fn main() {
    let handler = XmlHandler::new(false); 
    let input = 
    <?xml version="1.0"?>
    <!DOCTYPE root [
      <!ENTITY xxe SYSTEM "secret.txt">
    ]>
    <root>&xxe;</root>
    "#;
    match handler.parse(input) {
        Ok(result) => println!("Parsed: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
