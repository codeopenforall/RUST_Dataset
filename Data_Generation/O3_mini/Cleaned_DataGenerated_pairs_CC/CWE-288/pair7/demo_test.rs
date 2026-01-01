///////////////////////////////
// Test Oracle for Auth Check  //
// This test asserts that a request with an incorrect token should fail authentication.
///////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_authentication() {
        // Create an instance with the valid secret.
        let app = App::new("secret".to_string());
        
        // Prepare a request that includes the debug header.
        let req = Request {
            token: "wrong_secret".to_string(),
            headers: {
                let mut h = HashMap::new();
                h.insert("X-Debug".to_string(), "true".to_string());
                h
            },
        };

        // The result should be false (authentication fails) in the secure (fixed) version.
        let res = app.process_request(&req);
        assert_eq!(res, false, "Authentication bypassed when debug header is set");
    }
}