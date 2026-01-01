#[cfg(test)]
mod tests {
    use super::*;

    // Malicious input containing CRLF sequences intended to inject an extra header.
    fn injected_input() -> &'static str {
        "malicious\r\nX-Injection: attack"
    }

    #[test]
    fn test_header_injection() {
        let response = build_response(injected_input());
        // The vulnerable version will include the injected header when given unsanitized input.
        // The fixed version sanitizes the input so that the exact CRLF injection pattern is impossible.
        assert!(
            !response.get_header().contains("\r\nX-Injection:"),
            "Header injection detected"
        );
        assert!(response.get_header().contains("HTTP/1.1 200 OK"));
    }
}