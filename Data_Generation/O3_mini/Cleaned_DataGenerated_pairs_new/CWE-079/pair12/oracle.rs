#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_html_escape() {
        let input = "<script>alert('xss');</script>";
        let expected = "<html><body>User: &lt;script&gt;alert(&#x27;xss&#x27;);&lt;/script&gt;</body></html>";
        let engine = TemplateEngine::new("<html><body>User: {{input}}</body></html>");
        let output = engine.render(input);
        assert_eq!(output, expected, "HTML escaping failure, potential XSS vulnerability.");
    }
}
