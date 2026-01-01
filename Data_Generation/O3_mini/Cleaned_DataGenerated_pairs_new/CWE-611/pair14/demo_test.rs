//////////////////////////////////////////////
// Test Oracle for CWE-611 (XXE)
// This test function is designed to run against both the vulnerable and fixed versions.
// It provides an XML input containing an external entity definition and asserts that
// the processing function rejects the input (returns an Err). The fixed version should pass
// this test by rejecting the input, while the vulnerable version improperly processes the input,
// causing the test to fail.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reject_external_entity() {
        let xml_input = r#"
            <?xml version="1.0"?>
            <!DOCTYPE foo [
                <!ENTITY xxe SYSTEM "external.txt">
            ]>
            <foo>&xxe;</foo>
        "#;
        let processor = XmlProcessor::new();
        let result = processor.process(xml_input);
        // The secure, fixed version should return an error indicating that external entities are not allowed.
        assert!(result.is_err(), "XML input with external entity should be rejected");
    }
}