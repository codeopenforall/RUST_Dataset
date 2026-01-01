#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_sanitization() {
        let malicious = "<script>alert('XSS');</script>".to_string();
        let response = process_request(malicious);
        assert!(
            !response.contains("<script>"),
            "Unescaped script tag detected, potential XSS vulnerability."
        );
    }
}
