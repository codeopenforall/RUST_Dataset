#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let malicious = "<script>alert('XSS');</script>";
        let expected = "<html><body>User: &lt;script&gt;alert(&#x27;XSS&#x27;);&lt;/script&gt;</body></html>";
        let output = process_input(malicious);
        assert_eq!(output, expected, "The HTML output did not escape malicious input properly.");
    }
}
