/*
The following test oracle function is intended to be compiled separately from the main source.
It tests the HTTP header generation routine by supplying an input that includes CRLF sequences.
For the vulnerable version, the test will fail because the resulting header contains the injected line,
whereas in the fixed version, the injection is removed.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_header_sanitization() {
        let malicious_input = "SafeValue\r\nInjected: evil";
        let resp = create_response(malicious_input, "Test Body");
        // The fixed version should not contain the injected header part.
        assert!(!resp.headers.contains("\r\nInjected:"), "CRLF injection detected in HTTP header!");
    }
}