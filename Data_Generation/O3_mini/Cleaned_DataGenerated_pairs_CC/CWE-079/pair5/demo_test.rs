#[cfg(test)]
mod tests {
    // The tests here assume that the current Renderer implementation is the one under evaluation.
    use super::*;

    #[test]
    fn test_oracle() {
        // This test uses a malicious input designed to trigger an XSS vulnerability.
        // The secure version should escape the input and *not* contain a raw <script> tag.
        let renderer = Renderer::new();
        let malicious_input = "<script>alert(1)</script>";
        let rendered_html = renderer.build_page(malicious_input);

        // Check that the dangerous substring is not present.
        assert!(
            !rendered_html.contains("<script>"),
            "Output contains a raw script tag, indicating a vulnerability."
        );
        // Additionally, in the fixed version, the '<' should be escaped to "&lt;"
        assert!(
            rendered_html.contains("&lt;script&gt;"),
            "Output does not contain the properly escaped script tag."
        );
        // Verify the overall structure of the rendered HTML.
        assert!(
            rendered_html.starts_with("<html>") && rendered_html.ends_with("</html>"),
            "Output does not have a proper HTML structure."
        );
    }
}