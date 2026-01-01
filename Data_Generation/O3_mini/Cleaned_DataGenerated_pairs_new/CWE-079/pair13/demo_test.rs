// This test oracle verifies that the Renderer::render method properly escapes HTML
// in the fixed code and reflects raw input in the vulnerable code.
// For the vulnerable code, the test will fail because malicious script tags are present,
// while for the fixed code, the test will pass because the input is escaped.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle() {
        let engine = Renderer::new();
        let malicious = "<script>alert('xss')</script>";
        let output = engine.render(malicious);
        // In safe code the output should have escaped characters, e.g. &lt;script&gt;
        // Test expects that raw '<script>' is not present.
        assert!(
            !output.contains("<script>"),
            "Test failed: Unescaped script tag found in output."
        );
    }
}