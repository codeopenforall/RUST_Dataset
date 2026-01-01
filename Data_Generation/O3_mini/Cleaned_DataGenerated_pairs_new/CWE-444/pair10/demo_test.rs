//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        // Attack input with the extra malicious header.
        let malicious = "GET / HTTP/1.1\r\nContent-Length: 0\r\nX-Extra: exploit\r\n\r\nBODY";
        let req = Request { raw: malicious.to_owned() };
        
        // For a secure implementation, the malicious request should be rejected (i.e. process_request returns false).
        // In the vulnerable version, the unsafe behavior causes process_request to erroneously return true.
        // This test will fail for the vulnerable code and pass for the corrected code.
        assert!(!process_request(&req), "Malicious request should be rejected");
    }
}