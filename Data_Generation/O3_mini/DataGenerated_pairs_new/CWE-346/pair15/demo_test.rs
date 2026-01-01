////////////////////////////////////////////////////////////
// Test Oracle Function
////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // This test should fail for the vulnerable version
    // because it will process the unauthorized origin resulting in a constructed URL.
    // In the corrected version, the test passes as it returns an error.
    #[test]
    fn test_origin_validation() {
        // Create service with only "trusted.com" approved.
        let service = Service::new(vec!["trusted.com".to_string()]);
        let mut headers = HashMap::new();
        // Use a malicious origin that is not permitted.
        headers.insert("Origin".to_string(), "malicious.com".to_string());
        let req = Request::new(headers, String::new());
        let result = service.process(req);
        // For secure behavior, an error should be returned.
        assert!(
            result.is_err(),
            "Expected error for unauthorized origin, but got: {:?}",
            result
        );
    }
}