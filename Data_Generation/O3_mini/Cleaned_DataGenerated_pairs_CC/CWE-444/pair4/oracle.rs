#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_request_smuggling() {
        let malicious = "POST / HTTP/1.1\r\nHost: test\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n";
        let result = simulate(malicious);
        assert_eq!(result, 0, "Request with conflicting Content-Length headers should be rejected (return 0)");
    }
}
