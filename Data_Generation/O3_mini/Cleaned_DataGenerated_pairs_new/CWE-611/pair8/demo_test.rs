/////////////////////////////////////////////////////////////////
// Test Oracle Function: Verifying XXE Resolution Behavior
/////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use std::path::Path;
    
    // Import the Processor and Document types from the implementation.
    // The same test will be run against both versions to ensure that the vulnerability is present
    // in the vulnerable version (test should fail) and fixed in the corrected version (test should pass).
    use super::{Processor};

    #[test]
    fn test_external_entity_sanitization() {
        // Prepare the external file that would be targeted by the XXE.
        // In a real-world scenario, this file may contain sensitive information.
        let file_path = "vulnerable.txt";
        let expected_secret = "SECRET DATA";
        fs::write(&file_path, expected_secret).expect("Unable to create the external file");

        // The crafted XML input includes an external entity reference.
        let xml_data = r#"<?xml version="1.0"?>
<!DOCTYPE foo [
    <!ENTITY xxe SYSTEM "vulnerable.txt">
]>
<foo>&xxe;</foo>"#;
        
        let processor = Processor::new();
        let doc = processor.parse(xml_data);

        // In the safe (fixed) implementation, external entities are removed, so the output should 
        // NOT contain the secret from the external file.
        // For the vulnerable version, the content of vulnerable.txt would be ingested (or "Error" if file reading fails),
        // causing this assertion to fail if external entity resolution is not disabled.
        assert!(!doc.content.contains(expected_secret), "External entity was resolved, indicating the vulnerability");
        
        // Clean up the external file after testing.
        if Path::new(&file_path).exists() {
            fs::remove_file(&file_path).expect("Failed to remove the external file");
        }
    }
}