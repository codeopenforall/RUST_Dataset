#[cfg(test)]
mod tests {
    use super::server;
    
    // Test oracle: Send a malicious HTTP request with duplicate, conflicting Content-Length headers.
    // For the vulnerable version, the processing function accepts the first header (5) leading to an
    // inconsistent interpretation (since the actual body length is 11). The test expects an error.
    // For the corrected version, the function properly rejects the request due to inconsistent headers.
    #[test]
    fn test_request_smuggling() {
        // Duplicate headers: first is 5, second is 10 but actual body "hello world" is 11 bytes.
        let malicious_req = b"POST / HTTP/1.1\r\nHost: test\r\nContent-Length: 5\r\nContent-Length: 10\r\n\r\nhello world";
        
        // The contract: processing must return an error.
        let res = unsafe { server::process_request(malicious_req) };
        assert!(res.is_err(), "Request with duplicate conflicting headers must be rejected");
    }
}