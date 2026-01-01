///////////////////////////////////////////////////////////////
// Test Oracle Function for HTTP Header Injection
// This test function validates the behavior of the production code.
// It uses a malicious input that includes CRLF sequences to attempt header injection.
// The invariant is that the resulting header should NOT contain an injected header key.
// This oracle should FAIL when run against the vulnerable version and PASS against the fixed version.
///////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_header_sanitization() {
        // Malicious payload containing CRLF sequences.
        let payload = "malicious\r\nInjected: bad";
        let response = process(payload);

        // In a secure implementation, the response should not contain the string "Injected:".
        // This assertion expects that no new header was injected.
        assert!(
            !response.contains("\nInjected:"),
            "Header injection detected: response contains injected header"
        );
    }
}