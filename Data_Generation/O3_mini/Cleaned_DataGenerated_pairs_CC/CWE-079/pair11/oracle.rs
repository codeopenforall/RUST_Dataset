#[cfg(test)]
mod tests {
    use super::handle_input;
    #[test]
    fn test_xss_sanitization() {
        let malicious_input = "<script>alert('XSS')</script>";
        let result = handle_input(malicious_input.to_string());
        assert!(
            !result.contains("<script>"),
            "Output contains unescaped script tags, XSS vulnerability present"
        );
        assert!(
            result.contains("&lt;script&gt;"),
            "Output does not properly escape script tags"
        );
    }
}
