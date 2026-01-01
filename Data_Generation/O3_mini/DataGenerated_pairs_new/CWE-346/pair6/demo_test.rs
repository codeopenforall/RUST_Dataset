#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn header_validation_oracle() {
        // The test uses a header that starts with the trusted origin but includes extra malicious text.
        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), "https://trusted.com.evil".to_string());
        let req = Arc::new(Request { headers });
        let engine = Engine;
        let outcome = engine.run(req);
        // The test asserts that the header validation must fail.
        assert!(!outcome, "Header validation should fail for malicious origin input");
    }
}