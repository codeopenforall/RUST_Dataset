//////////////////////////////////////////////
// Test Oracle Function for Authentication Check
// This test should fail when run against the vulnerable version (debug endpoint bypasses auth)
// and pass with the fixed version (debug endpoint requires authentication).
//////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authentication_oracle() {
        // Initialize the server in authentication-required mode.
        let server = Server::new("supersecret", true);
        // Prepare a request to the debug endpoint without a token.
        let debug_request = Request {
            endpoint: "/debug".to_string(),
            token: None,
        };
        // The corrected behavior (fixed version) should reject the request.
        // Vulnerable version would incorrectly grant access.
        let access_granted = server.process_request(&debug_request);
        assert_eq!(
            access_granted,
            false,
            "Unauthorized access to the debug endpoint should be denied"
        );
    }
}