////////////////////////////////////////////////////////////////////////////////
// Test Oracle Function
// This test is designed to be run against both versions. For the fixed version,
// the test should pass, as it returns an error on detecting external entity reference,
// whereas the vulnerable version will attempt to resolve the external entity (possibly
// returning sensitive file content) and cause the test to fail.
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_oracle() {
        // Construct a malicious XML with external entity reference.
        let xml_input = "<!DOCTYPE test [ <!ENTITY external SYSTEM \"file:///secret.txt\"> ]><test>&external;</test>";
        let processor = Processor;
        let result = processor.process(xml_input);
        
        // The expected behavior for the secure implementation is to return an error.
        // Therefore, we assert that the result equals Err("External entities are disabled").
        assert_eq!(result, Err("External entities are disabled"),
            "The XML processor should reject external entities, but it did not.");
    }
}