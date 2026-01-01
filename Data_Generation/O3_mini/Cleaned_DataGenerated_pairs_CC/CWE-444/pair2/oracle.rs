#[cfg(test)]
mod tests {
    use super::*;
    fn oracle() {
        let malicious_request = "POST /upload HTTP/1.1\r\n\
                                 Host: example.com\r\n\
                                 Content-Length: 11\r\n\
                                 Transfer-Encoding: chunked\r\n\r\n\
                                 Hello World\r\n\
                                 GET /malicious HTTP/1.1\r\n\
                                 Host: evil.com\r\n\r\n";
        let result = execute_request(malicious_request);
        assert!(result.is_err(), "Ambiguous request was not rejected");
    }
    #[test]
    fn test_oracle() {
        oracle();
    }
}
