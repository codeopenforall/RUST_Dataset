//////////////////////////////////////////////
// Test Oracle Function
//
// The following test function is used to verify that the vulnerable code bypasses authentication
// when a debug header is present, while the corrected version enforces proper authentication.
//
// For both versions, a Request with the "X-Debug" header and an incorrect token should fail authentication.
// Also, a regular request with valid credentials should succeed.
//////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_authentication() {
        let svc = Service;
        // Test case: Debug header is present with an incorrect token.
        // In the vulnerable code, this would bypass auth and return true.
        // In the fixed code, this should return false.
        let req_debug = Request {
            headers: vec![("X-Debug".to_string(), "wrong".to_string())],
            username: "user".to_string(),
            password: "badpass".to_string(),
        };

        // Expectation: Authentication should fail for debug requests without a valid token.
        let result = svc.authenticate(&req_debug);
        assert!(!result, "Authentication bypass occurred via debug header!");

        // Normal authentication: Only valid credentials should pass.
        let req_normal = Request {
            headers: vec![],
            username: "admin".to_string(),
            password: "password".to_string(),
        };
        let result_normal = svc.authenticate(&req_normal);
        assert!(result_normal, "Valid authentication failed!");
    }
}