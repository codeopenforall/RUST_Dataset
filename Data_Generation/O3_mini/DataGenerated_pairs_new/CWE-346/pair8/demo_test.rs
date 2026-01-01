/////////////////////////////////////////////////////////////////
// Test Oracle Function
// This test function creates a Request with a dynamically allocated
// header value that matches "trusted.example.com". In the vulnerable
// version, the pointer comparison fails, so the function returns false,
// causing the test to fail. In the corrected version, the string content
// is properly compared, and the function returns true. The assertion below
// verifies the correctness of the processing function.
/////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_origin_validation() {
        let mut headers = HashMap::new();
        // Create a dynamically allocated string.
        headers.insert("host".to_string(), String::from("trusted.example.com"));
        let req = Request { headers: headers };
        let result = process_request(&req);
        assert!(result, "Origin validation failed: expected valid origin.");
    }
}