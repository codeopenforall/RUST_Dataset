#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_smuggling() {
        // Crafted malicious input with conflicting Content-Length headers.
        let malicious = "POST / HTTP/1.1\r\nHost: test\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n";

        // In the vulnerable implementation, the first header value (10) is returned,
        // which is inconsistent with security requirements.
        // The fixed implementation returns 0 when detecting conflicting header values.
        let result = simulate(malicious);
        assert_eq!(result, 0, "Request with conflicting Content-Length headers should be rejected (return 0)");
    }
}