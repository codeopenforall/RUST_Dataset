//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // This test is designed so that:
    // - In the vulnerable implementation, a request crafted with an untrusted origin (and missing the Authorization header)
    //   will incorrectly authenticate (i.e. return true).
    // - In the fixed implementation, the same request must not authenticate (i.e. return false).
    #[test]
    fn test_authentication_integrity() {
        // Simulate a request that an attacker might send in production.
        // It lacks an Authorization header and provides an untrusted "X-Origin".
        let req = Request {
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("X-Origin".to_string(), "production_host".to_string()),
            ],
        };
        // Even when debug is enabled (misconfigured in production),
        // authentication should fail without proper credentials.
        let handler = Handler::new(true, "secret");
        let result = handler.serve_request(req);
        // For the fixed code, the result should be false.
        // In the vulnerable code, due to the unconditional bypass when debug is enabled,
        // this test would fail (returning true).
        assert_eq!(result, false, "Authentication bypass vulnerability detected.");
    }
}