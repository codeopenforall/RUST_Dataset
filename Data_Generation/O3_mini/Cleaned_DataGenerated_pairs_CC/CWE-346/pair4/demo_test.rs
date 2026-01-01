//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // This test supplies a header with an embedded null byte.
    // For the vulnerable implementation, the unsafe CStr conversion would truncate the string
    // and mistakenly allow the request (test failure). The fixed version correctly rejects it.
    #[test]
    fn test_origin_validation() {
        let mut hdrs = HashMap::new();
        // Crafted input: The embedded null byte causes the CStr conversion
        // to only consider "https://example.com" in the vulnerable case.
        hdrs.insert("Origin".to_string(), "https://example.com\0.malicious".to_string());
        let req = Request { headers: hdrs };

        // The check should return false for a properly validated header.
        let result = req.check_origin();
        assert!(!result, "The request should be rejected due to invalid origin header.");
    }
}