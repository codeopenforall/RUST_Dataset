/*
   Test Oracle:
   This test function is designed to be run as part of a unit test suite.
   It creates a simulated request with the "Origin" header correctly set to the trusted domain 
   and a manipulated "X-Forwarded-Host" header set to an untrusted value.
   The expectation is that the proper (fixed) implementation will reject the request 
   (return false), while the vulnerable implementation would erroneously allow it.
   Do not include this oracle function in the main server code.
*/
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    // Assume that the Server struct and process_request function are accessible.

    // This oracle test is intended to be used with the fixed version of the code.
    // When run against the vulnerable version, it will fail.
    #[test]
    fn test_origin_validation() {
        // Create an instance of the server.
        let server = super::Server::new();
        let mut headers = HashMap::new();
        // Set the Origin header to the trusted URL.
        headers.insert("Origin".to_string(), "https://trusted.com".to_string());
        // Insert a malicious forwarded header.
        headers.insert("X-Forwarded-Host".to_string(), "https://evil.com".to_string());

        let allowed = server.process_request(&headers);
        assert!(!allowed, "The request should be rejected when a manipulated forwarded header is provided.");
    }
}