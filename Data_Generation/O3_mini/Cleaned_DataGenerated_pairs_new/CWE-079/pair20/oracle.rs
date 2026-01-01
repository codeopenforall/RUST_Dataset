#[cfg(test)]
mod tests {
    use super::Template;
    #[test]
    fn test_template_rendering() {
        let tmpl = Template::new();
        let malicious = <script>alert('XSS');</script>"#;
        let generated_html = tmpl.render(malicious);
        assert!(
            !generated_html.contains("<script>"),
            "Test failed: Unescaped script tag found in the output."
        );
    }
}
