///////////////////////////////////////////////////////////////
// Test Oracle: This unit test validates the HTML output by ensuring that
// the untrusted input is properly escaped. The test is expected to fail
// when using the vulnerable version (which outputs raw "<script>") and to
// pass with the fixed version (which escapes the script tags).
///////////////////////////////////////////////////////////////
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
        // The vulnerable implementation will include the raw "<script>" tag.
        // The fixed implementation should escape it to "&lt;script&gt;".
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