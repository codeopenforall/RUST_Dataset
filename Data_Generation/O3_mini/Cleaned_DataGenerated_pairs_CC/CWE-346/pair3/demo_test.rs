#[cfg(test)]
mod tests {
    use super::*;

    // Test contract:
    // When a request comes with a non-trusted origin, the secure version should reject it
    // (return an Err), while the vulnerable version would wrongly accept it (return Ok).
    //
    // Note: This test oracle is designed to be run separately against either the vulnerable or
    // fixed implementation. In a secure implementation, the test should pass; in a vulnerable
    // implementation, the test should fail.
    #[test]
    fn test_origin_validation() {
        // Create a request with a malicious origin
        let req = Request {
            header: "malicious.com".to_string(),
            body: "Attack payload".to_string(),
        };
        // The secure implementation is expected to reject the request.
        let result = process_request(req);
        assert!(
            result.is_err(),
            "Request with unauthorized origin should be rejected."
        );
    }
}