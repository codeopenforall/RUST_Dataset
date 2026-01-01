#[cfg(test)]
mod tests {
    // Import the Parser from the corresponding module.
    use super::Parser;

    #[test]
    fn test_oracle() {
        // Malicious XML input containing an external entity.
        let malicious_xml = r#"<?xml version="1.0"?>
<!DOCTYPE data [ <!ENTITY ext SYSTEM "external.txt"> ]>
<data>&ext;</data>"#;
        let parser = Parser;
        let result = parser.process(malicious_xml);
        // The intended behavior is to reject XML with external entities.
        // In the vulnerable variant, the process might incorrectly resolve the entity,
        // while in the safe variant it explicitly returns an error.
        assert!(result.is_err(), "External entity resolution should be rejected");
    }
}