//////////////////////////////////////////////
// Test Oracle: Authentication Bypass Check
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Common Request and Service definitions should match those in the main code.
    #[test]
    fn test_debug_endpoint_requires_valid_auth() {
        // Create an instance of the service with a known token.
        let svc = Service::new("secret");

        // Create a request for the debug endpoint without an "Authorization" header.
        let req_no_auth = Request {
            endpoint: "/debug",
            headers: HashMap::new(),
        };

        // The expected outcome is false (authentication should fail).
        // In the vulnerable version, this returns true due to the bypass.
        assert_eq!(svc.process(req_no_auth), false, "Authentication bypass occurred");

        // Create a request with an incorrect token.
        let mut headers_wrong = HashMap::new();
        headers_wrong.insert("Authorization".to_string(), "wrong".to_string());
        let req_wrong = Request {
            endpoint: "/debug",
            headers: headers_wrong,
        };
        assert_eq!(svc.process(req_wrong), false, "Incorrect token should not authenticate");

        // Create a request with the correct token.
        let mut headers_right = HashMap::new();
        headers_right.insert("Authorization".to_string(), "secret".to_string());
        let req_right = Request {
            endpoint: "/debug",
            headers: headers_right,
        };
        assert_eq!(svc.process(req_right), true, "Correct token must authenticate");
    }
}