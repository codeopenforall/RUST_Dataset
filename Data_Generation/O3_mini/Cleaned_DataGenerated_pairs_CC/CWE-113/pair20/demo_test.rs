//////////////////// Test Oracle ////////////////////

// The following test oracle simulates the injection attack.
// It passes a malicious input containing "\r\n" sequences to the responder.
// In the vulnerable implementation, the resulting response will contain an extra header "Set-Cookie", 
// whereas in the corrected version these characters are stripped, and no injection occurs.

#[cfg(test)]
mod tests {
    use super::*;
    
    // Utility function to simulate header building.
    fn build_test_response<F: Fn(&Responder, &str)>(append_fn: F) -> String {
        let responder = Responder::new();
        // Malicious input: attempts to inject an extra header.
        let input = "malicious\r\nSet-Cookie: session=123";
        append_fn(&responder, input);
        responder.build_response()
    }
    
    #[test]
    fn injection_test() {
        // Expected: The output should not contain a separate injected header.
        // The response should only contain one header line that does not include CR or LF.
        
        // Test with the vulnerable version simulation.
        let response_vulnerable = {
            // In the vulnerable version, no sanitization is done.
            // Calling the raw append_header directly.
            let responder = Responder::new();
            // Directly use vulnerable behavior.
            responder.append_header("X-Test", "malicious\r\nSet-Cookie: session=123");
            responder.build_response()
        };
        // The vulnerable response will include the injected header due to CRLF splitting.
        assert!(response_vulnerable.contains("Set-Cookie"), "Vulnerable version should be exploitable and include injected header");

        // Test with the corrected version simulation.
        let response_fixed = {
            // For the fixed version, we mimic calling the sanitized function.
            let responder = Responder::new();
            // Call the function that performs sanitization.
            responder.append_header("X-Test", "malicious\r\nSet-Cookie: session=123");
            responder.build_response()
        };
        // The fixed response should not contain any CR or LF, thus no header splitting.
        assert!(!response_fixed.contains("\r\nSet-Cookie"), "Fixed version should sanitize CRLF injection");
    }
}

// To run the tests:
// cargo test -- --nocapture