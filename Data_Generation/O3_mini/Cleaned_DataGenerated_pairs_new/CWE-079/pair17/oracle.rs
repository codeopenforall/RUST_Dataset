#[cfg(test)]
mod tests {
    use super::Renderer;
    #[test]
    fn test_html_escaping() {
        let renderer = Renderer::new();
        let malicious_input = <script>alert("XSS")</script>"#;
        let output = renderer.render(malicious_input);
        assert!(
            !output.contains("<script>"),
            "HTML output contains unescaped script tag, indicating an XSS vulnerability."
        );
        assert!(
            output.contains("&lt;script&gt;"),
            "HTML output does not appear to properly escape the malicious input."
        );
    }
}
