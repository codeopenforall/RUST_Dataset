///////////////////////////////
// Test Oracle Function
///////////////////////////////
#[cfg(test)]
mod tests {
    use super::build_response;

    #[test]
    fn test_header_injection() {
        // Malicious input contains CRLF characters intended for header splitting.
        let malicious = "GoodHeader\r\nInjected: evil-value";
        let header = build_response(malicious);
        // The test asserts that the header does not contain a CRLF followed by "Injected:".
        // For the vulnerable code, this assertion will fail, while the fixed code passes.
        assert!(
            !header.contains("\r\nInjected:"),
            "CRLF injection vulnerability detected!"
        );
    }
}