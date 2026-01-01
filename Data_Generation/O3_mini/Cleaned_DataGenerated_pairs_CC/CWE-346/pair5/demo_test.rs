//////////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // The following test should FAIL when run against the vulnerable implementation,
    // since the malicious header "https://trusted.com\0.evil" will be incorrectly validated as allowed.
    // In the fixed implementation, the full string does not match and the request is correctly blocked.
    #[test]
    fn origin_validation_test() {
        let engine = Engine::new("https://trusted.com");
        // Malicious header designed to bypass unsafe C-style conversion.
        let req = Request {
            origin: "https://trusted.com\0.evil".to_string(),
            body: "malicious payload".to_string(),
        };
        let result = engine.process(req);
        // The expected behavior is to block the request, so process() should return false.
        assert!(!result, "Origin header spoofing vulnerability: request should be blocked");
    }
}
//////////////////////////////////////////////
// End of Test Oracle
//////////////////////////////////////////////