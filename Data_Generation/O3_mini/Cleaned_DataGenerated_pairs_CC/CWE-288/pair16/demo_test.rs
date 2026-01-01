#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    // Import the common components from the module.
    use super::{Request, process_input, SecurityHandler};

    #[test]
    fn test_auth_fail_on_bypass() {
        // Construct a request that attempts to trigger authentication bypass.
        let mut headers = HashMap::new();
        headers.insert("X-Debug-Bypass".to_string(), "true".to_string());
        let req = Request {
            headers,
            body: "wrong-password".to_string(),
        };

        // For the fixed implementation, this should fail authentication.
        let handler = Arc::new(SecurityHandler::new());
        let result = process_input(handler, req);

        // Assert that the authentication fails (i.e. access is denied).
        assert!(!result, "Authentication bypass allowed access with debug header.");
    }
}