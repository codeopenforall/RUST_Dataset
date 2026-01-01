#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Helper function to set up the test file.
    fn setup_file() {
        fs::write("test.txt", "TOP_SECRET").expect("Unable to create test file");
    }
    
    // Helper function to clean up the test file.
    fn cleanup_file() {
        let _ = fs::remove_file("test.txt");
    }
    
    #[test]
    fn test_oracle() {
        setup_file();
        let xml_data = r#"<!DOCTYPE data [
            <!ENTITY ext SYSTEM "test.txt">
        ]>
        <data>&ext;</data>"#;
        
        let processor = XmlProcessor::new();
        let result = processor.process(xml_data);
        
        // For the secure version, external entity resolution is disabled and an error is returned.
        // The test expects an error. If external entity content is returned, that indicates vulnerability.
        match result {
            Ok(content) => {
                // If content matches the secret, then external entity was improperly processed.
                assert_ne!(content, "TOP_SECRET", "Test failed: External entity was resolved.");
            },
            Err(err) => {
                assert_eq!(err, "External entity resolution is disabled", "Test failed: Unexpected error message.");
            }
        }
        cleanup_file();
    }
}