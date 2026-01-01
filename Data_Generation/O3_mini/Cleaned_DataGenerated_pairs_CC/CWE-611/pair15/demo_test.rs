//////////////////////////////////////////////
// Test Oracle
// This test validates the functionality of the fixed implementation versus the vulnerable one.
// It uses a predetermined malicious XML input containing an external entity declaration.
// The expected behavior is that the safe (fixed) implementation will return 
// "Rejected external entity", while the vulnerable implementation would erroneously return sensitive data.
// When running this test against the vulnerable executable, the assertion will fail.
//////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Bring the ConfigProcessor into scope.
    use super::ConfigProcessor;

    #[test]
    fn test_external_entity_rejection() {
        // Malicious XML input aiming to trigger the external entity resolution.
        let malicious_input = "<!DOCTYPE config [<!ENTITY exploit SYSTEM \"file:///etc/passwd\">]>";
        let processor = ConfigProcessor::new();
        let result = processor.process(malicious_input);
        // The fixed implementation should reject the external entity resolution.
        assert_eq!(result, "Rejected external entity", "External entity resolution not blocked");
    }
}