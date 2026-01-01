////////////////////////////////////////////////////////////////////////
// Test oracle function for validating template rendering security.
// The following unit test is designed to fail when the rendered HTML contains
// unsanitized '<script>' tags (as in the vulnerable version) and to pass when
// proper escaping is applied (as in the fixed version).
//
// To run this test, include it into your test suite (e.g., using `cargo test`).
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_escaping() {
        let renderer = Renderer::new();
        let malicious_input = "<script>alert('XSS')</script>";
        let result = renderer.render(malicious_input);
        // Assert that the rendered output does not contain the raw "<script>" tag.
        assert!(!result.contains("<script>"), "Rendered output must not include raw script tags");
        // Additionally, verify that the escaped version is present.
        assert!(result.contains("&lt;script&gt;"), "Rendered output should escape script tags");
    }
}