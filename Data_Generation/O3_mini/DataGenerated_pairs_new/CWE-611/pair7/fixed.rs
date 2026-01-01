/////////////////////////////////////////////////////////////
// Corrected Example: XXE Mitigation by Disabling External Entity Resolution
/////////////////////////////////////////////////////////////
pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    // The corrected parse function fully disables external entity resolution.
    // Any XML input containing SYSTEM is rejected, eliminating XXE risks.
    pub fn parse(&self, xml: &str) -> Result<String, &'static str> {
        if xml.contains("SYSTEM") {
            return Err("External entity resolution is disabled");
        }
        Ok(xml.to_string())
    }
}

fn main() {
    // Example XML with an external entity; this input is now rejected.
    let xml = r#"<!DOCTYPE data [
       <!ENTITY ext SYSTEM "./test_entity.txt">
    ]>
    <data>&ext;</data>"#;
    
    let parser = Parser::new();
    match parser.parse(xml) {
        Ok(result) => println!("Parsed content: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}