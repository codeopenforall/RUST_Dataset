#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validate_html_encoding() {
        let page = Page {
            header: "<html><head><title>Test</title></head><body>".to_string(),
            content: "</body></html>".to_string(),
        };
        let injection = "<script>alert('XSS');</script>";
        let output = page.render(injection);
        assert!(
            !output.contains("<script>"),
            "Output contains unescaped script tag!"
        );
        assert!(
            output.contains("&lt;script&gt;"),
            "Output does not contain escaped script tag!"
        );
    }
}
