/*
This test oracle validates the contract across both versions. It constructs a Request
with a malicious Origin header ("http://evil.com"). The expected behavior is to reject
the request by returning "Rejected". The insecure/vulnerable version erroneously accepts
such requests (returning "Accepted"), causing the test to fail; the fixed version passes.
*/
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_origin_validation() {
        let req = Request {
            origin: "http://evil.com".to_string(),
            path: "/test".to_string(),
        };
        let result = process_request(req);
        assert_eq!(result, "Rejected", "Request with malicious origin should be rejected");
    }
}