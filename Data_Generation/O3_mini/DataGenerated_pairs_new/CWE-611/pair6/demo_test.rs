//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use std::fs;
    use super::Document;

    #[test]
    fn test_external_entity_resolution() {
        // Setup: Create external.txt with a known payload.
        let payload = "malicious payload";
        fs::write("external.txt", payload).expect("Unable to write external.txt");

        let input_xml = r#"
        <?xml version="1.0"?>
        <!DOCTYPE sample [
          <!ENTITY ext SYSTEM "external.txt">
        ]>
        <sample>&ext;</sample>
        "#;

        let doc = Document::new(input_xml);

        // In the insecure implementation, the external file is resolved.
        // In the secure implementation, the XML is returned unmodified.
        // Here, we assert that external entity resolution did NOT occur.
        // If the content contains the payload, then the vulnerability is present.
        assert!(
            !doc.content.contains(payload),
            "External entity was improperly resolved."
        );
    }
}
//////////////////////////////
// End Test Oracle Function
//////////////////////////////