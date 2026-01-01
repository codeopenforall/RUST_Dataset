#[cfg(test)]
mod tests {
    use super::process_request;
    #[test]
    fn test_oracle() {
        let malicious_request = "POST / HTTP/1.1\r\n\
                                 Host: example.com\r\n\
                                 Content-Length: 5\r\n\
                                 Content-Length: 10\r\n\
                                 \r\n\
                                 helloWorld";
        let result = process_request(malicious_request);
        assert!(result.is_err(), "Request with duplicate Content-Length headers should be rejected");
    }
}
