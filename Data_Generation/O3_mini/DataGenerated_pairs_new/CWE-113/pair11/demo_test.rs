///////////////////////////////////////////////////////////////////////
// Test Oracle: This unit test fails when CRLF injection is present and
// passes when the header value is correctly sanitized.
// This test should be placed in a separate test module.
///////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_integrity() {
        // Input containing CRLF attempting to inject an extra header.
        let attacker_input = "Value\r\nInjected-Header: injected";
        let server = Server::initiate();
        
        // Directly call the unsafe method.
        unsafe {
            server.append("X-Test", attacker_input);
        }
        
        let headers = server.retrieve();
        // The test asserts that none of the headers contain CR or LF characters.
        for header in headers {
            assert!(!header.contains('\r'), "Header contains CR");
            assert!(!header.contains('\n'), "Header contains LF");
        }
    }
}