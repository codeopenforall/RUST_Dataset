#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // This test verifies that providing only an incorrect debug header does not grant access.
    // The expected response is "Unauthorized". In the vulnerable code, the presence of any
    // "X-Debug" header would cause an authentication bypass, leading to a failure of this test.
    #[test]
    fn test_no_auth_bypass() {
        let auth = TokenAuthenticator::new("secret123", "debug_secret");
        let api = Api::new(Box::new(auth));
        let mut headers = HashMap::new();
        // Provide an incorrect debug header value (should be "debug_secret" for valid access)
        headers.insert("X-Debug".to_string(), "true".to_string());
        let response = api.process_request(headers);
        assert_eq!(response, "Unauthorized", "Authentication bypass occurred!");
    }
}