use std::fs;
struct Document {
    content: String,
}
struct Processor;
impl Processor {
    fn new() -> Self {
        Processor
    }
    fn parse(&self, input: &str) -> Document {
        if input.contains("<!ENTITY") && input.contains("SYSTEM") {
            let sanitized = input.replace(<!ENTITY xxe SYSTEM "vulnerable.txt">"#, "");
            return Document {
                content: sanitized,
            };
        }
        Document {
            content: input.to_string(),
        }
    }
}
fn main() {
    let xml_data = <?xml version="1.0"?>
<!DOCTYPE foo [
    <!ENTITY xxe SYSTEM "vulnerable.txt">
]>
<foo>&xxe;</foo>"#;
    let processor = Processor::new();
    let doc = processor.parse(xml_data);
    println!("Parsed content: {}", doc.content);
}
