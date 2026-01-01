#[cfg(test)]
mod tests {
    use super::*;

    // This test ensures that the produced header does not contain a newline followed by an injected header.
    // In the vulnerable version, the CRLF injection will cause the header to contain "Injected-Header:" on a new line.
    #[test]
    fn test_prevent_injection() {
        // Malicious input designed to perform header injection.
        let input = "http://example.com\r\nInjected-Header: injected_value".to_string();
        let result = simulate(input);
        // The test asserts that the response must not contain a CRLF immediately preceding "Injected-Header:".
        assert!(
            !result.contains("\r\nInjected-Header:"),
            "Header injection vulnerability present"
        );
    }
}