///////////////////////////////////////////////////////////
// Test Oracle Function:
//
// This test function uses a predetermined XML input containing
// an external entity reference. It calls the XML processor and
// asserts that the output is an error with the specific message
// "External entities not allowed". In the insecure implementation,
// the external entity is processed (or an alternate error message
// is produced), and the test will fail. In the secure implementation,
// the test will pass.
///////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_external_entity_rejection() {
        let processor = XmlProcessor;
        let input = "<root>&ext;</root>";
        let res = processor.process(input);
        // The expected outcome is an error indicating that external entity processing is disallowed.
        assert!(res.is_err(), "The XML processor should reject external entities");
        assert_eq!(res.err().unwrap(), "External entities not allowed");
    }
}