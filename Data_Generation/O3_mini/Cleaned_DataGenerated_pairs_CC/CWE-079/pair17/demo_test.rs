/*
Test Oracle: This standalone test verifies that the HTML renderer properly escapes malicious inputs.
It instantiates the Renderer, renders a malicious input and asserts that the raw <script> tag 
does not appear in the resulting HTML.

Compile and run this test with the Rust test harness (e.g. `cargo test`). The test should fail
when run against the vulnerable implementation and pass when run against the corrected version.
*/

#[cfg(test)]
mod tests {
    // Import the Renderer from the current crate.
    use super::Renderer;

    #[test]
    fn test_html_escaping() {
        let renderer = Renderer::new();
        let malicious_input = r#"<script>alert("XSS")</script>"#;
        let output = renderer.render(malicious_input);
        // The test expects that the output does not contain any raw "<script>" tags.
        assert!(
            !output.contains("<script>"),
            "HTML output contains unescaped script tag, indicating an XSS vulnerability."
        );
        // Additionally, ensure that the input has been escaped.
        assert!(
            output.contains("&lt;script&gt;"),
            "HTML output does not appear to properly escape the malicious input."
        );
    }
}