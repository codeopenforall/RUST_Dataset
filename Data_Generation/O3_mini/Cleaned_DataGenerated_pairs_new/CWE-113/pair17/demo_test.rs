#[cfg(test)]
mod tests {
    use super::*;

    // This test uses a malicious input containing CRLF sequences.
    // The expected correct output should have these characters removed.
    #[test]
    fn test_response_safety() {
        let responder = Responder::new();
        let malicious_input = "Injected: test\r\nAnother: header\r\n";
        let response = responder.build(malicious_input);
        // The baseline header already contains CRLF characters per HTTP standards.
        // The malicious portion should be sanitized to remove any CR and LF.
        let expected_suffix = "Injected: testAnother: header";
        assert!(response.ends_with(expected_suffix), "HTTP header injection vulnerability detected.");
    }
}