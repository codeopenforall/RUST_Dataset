#[cfg(test)]
mod tests {
    use super::*;
    
    // Test oracle: When processing an input below the threshold (20),
    // the operation must be denied. For the flawed version, the test will fail
    // because it incorrectly grants access; for the corrected version, the test will pass.
    #[test]
    fn test_security_validation() {
        let processor = Processor::new(50);
        let result = processor.process(20);
        // We expect an error ("Access Denied") for unauthorized access.
        assert!(result.is_err(), "Unauthorized access granted when it should be denied.");
        if let Err(err) = result {
            assert_eq!(err, "Access Denied");
        }
    }
}