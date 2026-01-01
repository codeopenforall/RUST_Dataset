//////////////// Test Oracle ////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // This test ensures that requests with an incorrect origin are rejected.
    // In the vulnerable implementation, the request would improperly be processed.
    // The fixed implementation should correctly return false for malicious headers.
    #[test]
    fn test_origin_validation() {
        let server = Server {
            allowed_origin: "https://secure.example.com".to_string(),
        };

        let req = Request {
            headers: {
                let mut h = HashMap::new();
                h.insert("Origin".to_string(), "https://malicious.example.com".to_string());
                h
            },
            body: "Test body".to_string(),
        };

        // The contract: the server should reject the request with a malicious origin.
        assert_eq!(server.process(&req), false, "Origin validation failed: malicious origin was accepted.");
    }
}