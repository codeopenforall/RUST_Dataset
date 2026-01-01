//////////////////////////////////////////////////////////////////
// Test Oracle: This unit test ensures that external entity resolution
// does not occur. For the insecure (vulnerable) implementation, the test
// will fail because the output includes "SensitiveDataFromFile". For the
// secure (fixed) version, the test passes as the external entity remains unresolved.
//////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_external_entity_resolution() {
        let xml_input = r#"<!DOCTYPE data [
<!ENTITY xxe SYSTEM "file:///etc/passwd">
]>
<data>&xxe;</data>"#;
        let parser = Parser::new();
        let result = parser.process(xml_input);
        // The secure behavior is to not substitute the external entity.
        assert!(
            !result.contains("SensitiveDataFromFile"),
            "Vulnerability detected: External entity resolution occurred"
        );
    }
}