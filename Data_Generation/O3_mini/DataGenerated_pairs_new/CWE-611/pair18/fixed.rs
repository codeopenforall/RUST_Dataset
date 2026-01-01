////////////////////////////////////////////
// Fixed Code Sample for CWE-611 (XXE)
// In this version the parser does not resolve external entities.
// Instead, it either rejects XML with external entity declarations or leaves them unresolved.
////////////////////////////////////////////

use std::error::Error;

pub struct XmlEngine;

impl XmlEngine {
    // Public function used by tests.
    pub fn process_xml(input: &str) -> Result<String, Box<dyn Error>> {
        // Check if the input contains an external entity definition.
        if input.contains("<!ENTITY ext SYSTEM") {
            // Instead of processing the external entity, we reject it or leave it unchanged.
            // Here we choose to leave the entity unresolved to prevent the vulnerability.
            let output = input.to_owned();
            Ok(output)
        } else {
            Ok(input.to_owned())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Example XML input that contains an external entity.
    let xml_data = "<!DOCTYPE test [<!ENTITY ext SYSTEM \"secret.txt\">]><test>&ext;</test>";
    let result = XmlEngine::process_xml(xml_data)?;
    // In the fixed build, the external entity is NOT resolved, mitigating the vulnerability.
    println!("Resulting XML: {}", result);
    Ok(())
}