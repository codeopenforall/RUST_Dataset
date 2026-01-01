#[cfg(test)]
mod tests {
    use super::*;

    // A common test oracle for XSS payload.
    // For a malicious payload, the vulnerable version would reflect the raw script tag,
    // while the safe version escapes it, preventing execution in a browser.
    #[test]
    fn test_xss_mitigation() {
        // Malicious user input designed to trigger XSS if unescaped.
        let malicious_input = "<script>alert('xss');</script>";
        let output = render(malicious_input);

        // The fixed version should not contain raw '<script>' tags.
        assert!(
            !output.contains("<script>"),
            "Detected raw script tags; potential XSS vulnerability."
        );
        // Additionally, check that the input has been escaped.
        assert!(
            output.contains("&lt;script&gt;"),
            "Input was not properly escaped."
        );
    }
}