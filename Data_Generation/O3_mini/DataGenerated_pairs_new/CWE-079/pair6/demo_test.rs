////////////////////////////
// Test Oracle Function
////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    // This test checks that the processed HTML output properly escapes untrusted input.
    // The test should FAIL for the vulnerable version (which reflects raw input)
    // and PASS for the corrected version (which escapes input).
    #[test]
    fn test_oracle() {
        let malicious = "<script>alert('XSS');</script>";
        // Expected output: untrusted characters escaped
        let expected = "<html><body>User: &lt;script&gt;alert(&#x27;XSS&#x27;);&lt;/script&gt;</body></html>";
        let output = process_input(malicious);
        assert_eq!(output, expected, "The HTML output did not escape malicious input properly.");
    }
}