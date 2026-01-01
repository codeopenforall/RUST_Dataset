#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let renderer = Renderer::new();
        let malicious_input = "<script>alert(1)</script>";
        let rendered_html = renderer.build_page(malicious_input);
        assert!(
            !rendered_html.contains("<script>"),
            "Output contains a raw script tag, indicating a vulnerability."
        );
        assert!(
            rendered_html.contains("&lt;script&gt;"),
            "Output does not contain the properly escaped script tag."
        );
        assert!(
            rendered_html.starts_with("<html>") && rendered_html.ends_with("</html>"),
            "Output does not have a proper HTML structure."
        );
    }
}
