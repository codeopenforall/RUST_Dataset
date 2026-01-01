#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_request_smuggling_protection() {
        let malicious = "POST / HTTP/1.1\r\nContent-Length: 5\r\nContent-Length: 10\r\n\r\nHello";
        let result = process_request(malicious);
        assert!(
            result.is_err(),
            "Request with duplicate Content-Length headers should be rejected"
        );
    }
}
