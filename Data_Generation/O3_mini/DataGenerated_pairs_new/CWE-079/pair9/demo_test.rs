#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xss_prevention() {
        // Create a test instance of the application.
        let app = App::new();
        // Define an attack payload that includes a script tag.
        let attack_payload = "<script>alert('XSS');</script>".to_string();
        // Render the output with the given payload.
        let output = app.handle(attack_payload.clone());
        
        // In the vulnerable version, the raw payload would be present in the output,
        // whereas in the corrected version, the payload should be properly escaped.
        // The test expects the output NOT to contain the literal "<script>" substring.

        // If the output does not contain escaped content, then the unsafe reflection is still active.
        if !output.contains("&lt;script&gt;") {
            panic!("Test failed: Unsanitized script tag detected in the rendered output.");
        }
        // Additionally, ensure that the raw unescaped tag is not present.
        assert!(!output.contains("<script>"), "Raw script tag should not be present in the output.");
    }
}