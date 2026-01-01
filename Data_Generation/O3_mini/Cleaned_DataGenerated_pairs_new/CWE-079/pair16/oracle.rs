#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_xss_mitigation() {
        let malicious_input = "<script>alert('xss');</script>";
        let output = render(malicious_input);
        assert!(
            !output.contains("<script>"),
            "Detected raw script tags; potential XSS vulnerability."
        );
        assert!(
            output.contains("&lt;script&gt;"),
            "Input was not properly escaped."
        );
    }
}
