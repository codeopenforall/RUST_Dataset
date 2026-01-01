//////////////////////////////////////////////////////////////////
// Test Oracle - Should fail when linked against the vulnerable code
// and pass when linked against the corrected code.
//////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn origin_validation() {
        let app = Application::new();
        let mut headers = HashMap::new();
        // Trigger - a malicious header input
        headers.insert("Origin".to_string(), "http://malicious.com".to_string());
        let req = Request { headers };
        let result = app.process(req);
        // The expected behavior is to reject the malicious origin.
        // Note: In the vulnerable code, this test would fail because the result would be true.
        assert_eq!(result, false, "The origin validation should fail for untrusted origins");
    }
}