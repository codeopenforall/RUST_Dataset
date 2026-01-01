//////////////////////
// Test Oracle Function
//////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crlf_injection_prevention() {
        // Malicious input containing CRLF injection.
        let malicious_input = "https://example.com\r\nInjected: malicious";
        let header = generate_response(malicious_input);
        // The fixed version must never include an injected header part.
        // For the vulnerable version this assertion would fail.
        assert!(
            !header.contains("\r\nInjected:"),
            "CRLF injection detected in header"
        );
    }
}