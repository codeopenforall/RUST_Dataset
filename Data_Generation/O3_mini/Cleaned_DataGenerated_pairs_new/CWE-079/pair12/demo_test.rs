#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_escape() {
        // The malicious input (commonly used for testing XSS vulnerabilities)
        let input = "<script>alert('xss');</script>";
        // The expected output after escaping: note that all special HTML characters are replaced with entities.
        let expected = "<html><body>User: &lt;script&gt;alert(&#x27;xss&#x27;);&lt;/script&gt;</body></html>";
        let engine = TemplateEngine::new("<html><body>User: {{input}}</body></html>");
        let output = engine.render(input);
        // In the safe version, the rendered HTML will match the expected escaped string.
        // In the vulnerable version, the test will fail.
        assert_eq!(output, expected, "HTML escaping failure, potential XSS vulnerability.");
    }
}