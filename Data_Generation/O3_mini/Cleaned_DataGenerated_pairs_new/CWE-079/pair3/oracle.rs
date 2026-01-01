#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_html_escaping() {
        let renderer = Renderer::new();
        let malicious_input = "<script>alert('XSS')</script>";
        let result = renderer.render(malicious_input);
        assert!(!result.contains("<script>"), "Rendered output must not include raw script tags");
        assert!(result.contains("&lt;script&gt;"), "Rendered output should escape script tags");
    }
}
