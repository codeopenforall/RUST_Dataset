//////////////////////////////////////////////
// Test Oracle: Verifies the request processing behavior.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // This test uses an ambiguous request that includes both
    // "Content-Length" and "Transfer-Encoding" headers. In the secure 
    // implementation, such a request must be rejected.
    fn oracle() {
        let malicious_request = "POST /upload HTTP/1.1\r\n\
                                 Host: example.com\r\n\
                                 Content-Length: 11\r\n\
                                 Transfer-Encoding: chunked\r\n\r\n\
                                 Hello World\r\n\
                                 GET /malicious HTTP/1.1\r\n\
                                 Host: evil.com\r\n\r\n";
        // The contract: the secure (fixed) version must return an error.
        let result = execute_request(malicious_request);
        assert!(result.is_err(), "Ambiguous request was not rejected");
    }

    #[test]
    fn test_oracle() {
        oracle();
    }
}
//////////////////////////////////////////////
// Note: When running the vulnerable version, the oracle test will fail
// since the ambiguous request is accepted. The fixed version will pass.
//////////////////////////////////////////////