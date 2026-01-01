pub struct Parser;
impl Parser {
    pub fn new() -> Self {
        Parser
    }
    pub fn parse(&self, xml: &str) -> Result<String, &'static str> {
        if xml.contains("SYSTEM") {
            return Err("External entity resolution is disabled");
        }
        Ok(xml.to_string())
    }
}
fn main() {
    let xml = <!DOCTYPE data [
       <!ENTITY ext SYSTEM "./test_entity.txt">
    ]>
    <data>&ext;</data>"#;
    let parser = Parser::new();
    match parser.parse(xml) {
        Ok(result) => println!("Parsed content: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
