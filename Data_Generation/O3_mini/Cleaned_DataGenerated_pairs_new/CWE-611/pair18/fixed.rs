use std::error::Error;
pub struct XmlEngine;
impl XmlEngine {
    pub fn process_xml(input: &str) -> Result<String, Box<dyn Error>> {
        if input.contains("<!ENTITY ext SYSTEM") {
            let output = input.to_owned();
            Ok(output)
        } else {
            Ok(input.to_owned())
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let xml_data = "<!DOCTYPE test [<!ENTITY ext SYSTEM \"secret.txt\">]><test>&ext;</test>";
    let result = XmlEngine::process_xml(xml_data)?;
    println!("Resulting XML: {}", result);
    Ok(())
}
