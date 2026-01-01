#[cfg(test)]
mod tests {
    use super::MyParser;
    use super::SimpleParser;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_xml_external_entity() {
        // Setup: Create a file that would be included by the external entity.
        let filename = "secret.txt";
        let secret_content = "TOP_SECRET_DATA";
        {
            let mut file = File::create(filename)
                .expect("Failed to create test file for external entity");
            write!(file, "{}", secret_content)
                .expect("Failed to write to test file for external entity");
        }

        // XML input with external entity referencing the created file.
        let xml_input = format!(r#"<?xml version="1.0"?>
<!DOCTYPE root [
<!ENTITY external SYSTEM "{}">
]>
<root>
    &external;
</root>"#, filename);

        // Parsing the XML.
        let result = MyParser::parse_xml(&xml_input);

        match result {
            Ok(content) => {
                // In the vulnerable version, the external file content is injected.
                // This should trigger a test failure because external entity resolution is unsafe.
                assert!(
                    !content.contains(secret_content),
                    "Test failed: External entity was resolved unsafely."
                );
            },
            Err(err) => {
                // In the secure implementation, an error is returned.
                assert_eq!(
                    err,
                    "External entities are not allowed",
                    "Test passed: External entities correctly rejected."
                );
            }
        }
    }
}