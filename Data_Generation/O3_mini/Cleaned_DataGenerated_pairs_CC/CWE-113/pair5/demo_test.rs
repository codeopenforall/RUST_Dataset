//////////////////////////////////////////////
// Test Oracle: Header sanitization test
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_oracle() {
        // Use an input containing CRLF injection payload.
        let input = "malicious\r\nInjected: bad";
        // Expected header after proper sanitization:
        // CR and LF characters are removed from the injected value.
        let expected = "HTTP/1.1 200 OK\r\nCustom-Header: maliciousInjected: bad\r\n";
        
        // Create a server instance using the input.
        let srv = Server::new(input);
        let header = srv.send();
        
        // Assert that the header output matches the expected sanitized value.
        assert_eq!(header, expected, "Header does not match expected sanitized output");
    }
}