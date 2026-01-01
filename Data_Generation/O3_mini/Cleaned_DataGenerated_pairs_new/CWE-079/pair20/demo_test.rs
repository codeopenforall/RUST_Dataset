#[cfg(test)]
mod tests {
    use super::Template;
    
    // The test oracle checks that the generated HTML does not contain unescaped script tags.
    // It fails if the output contains "<script>".
    #[test]
    fn test_template_rendering() {
        let tmpl = Template::new();
        let malicious = r#"<script>alert('XSS');</script>"#;
        let generated_html = tmpl.render(malicious);
        // The test ensures that the output does not include raw script tags.
        // In the vulnerable version, this assertion will fail.
        assert!(
            !generated_html.contains("<script>"),
            "Test failed: Unescaped script tag found in the output."
        );
    }
}