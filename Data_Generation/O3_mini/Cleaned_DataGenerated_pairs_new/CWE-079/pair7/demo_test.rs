#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn oracle() {
        // Test input that simulates an XSS payload.
        let renderer = Renderer::new();
        let input = "<script>alert('XSS');</script>";
        let output = renderer.generate(input);

        // Expected output for the corrected version should have escaped HTML entities.
        let expected = "<html><body>&lt;script&gt;alert(&#x27;XSS&#x27;);&lt;/script&gt;</body></html>";
        assert_eq!(output, expected, "User input was not properly escaped; potential XSS vulnerability remains.");
    }
}