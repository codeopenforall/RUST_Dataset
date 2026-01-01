////////////////////////////////////
// Test Oracle                    //
// This single unit test constructs //
// a request with a malicious value //
// for the Origin header. The test  //
// expects "Access denied" to be     //
// returned. The vulnerable version  //
// erroneously returns "Access granted"//
// causing the test to fail, while   //
// the fixed version passes.         //
////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_validation() {
        // Craft a request with a malicious origin.
        let malicious_request = HttpRequest::new(
            vec![("Origin".to_string(), "http://malicious.example.com".to_string())],
            "payload".to_string()
        );

        let server = Server::new();
        let result = server.run(malicious_request);
        // Expect rejection: "Access denied".
        assert_eq!(result, "Access denied", "The request must be rejected due to invalid origin");
    }
}