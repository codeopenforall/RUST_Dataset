#[cfg(test)]
mod tests {
    use super::*;
    
    // The test verifies that the rendered HTML does not contain raw <script> tags.
    // In the insecure version, the test will fail because the output retains the injection.
    // In the corrected version, the < and > characters are escaped.
    #[test]
    fn test_input_sanitization() {
        let malicious = "<script>alert('XSS');</script>".to_string();
        let response = process_request(malicious);
        // Assert that the output does not contain any raw script tags.
        assert!(
            !response.contains("<script>"),
            "Unescaped script tag detected, potential XSS vulnerability."
        );
    }
}