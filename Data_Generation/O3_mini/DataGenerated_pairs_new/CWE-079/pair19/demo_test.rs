//////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Oracle: This function acts as a unit test to verify the remediation for the Cross-Site Scripting issue.
// The oracle invokes the page assembling function with a malicious payload. For the vulnerable version, the
// unsanitized payload (including "<script>") would be present in the output, causing the test to fail. Conversely,
// for the fixed version, the payload will be escaped (e.g., "&lt;script&gt;"), ensuring the test passes.
//////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Page;

    #[test]
    fn verify_protection() {
        // The malicious payload simulating an XSS attack.
        let malicious = "<script>alert('xss');</script>";
        let page = Page::new("<html><body>");
        let output = page.assemble(malicious);
        // The test asserts that the output does not contain the literal "<script>" substring.
        // In the vulnerable version, this assertion would fail.
        assert!(!output.contains("<script>"), "Output contains unsanitized script tag");
        // Optionally, you can assert that the output contains the escaped version.
        assert!(output.contains("&lt;script&gt;"), "Output does not contain properly escaped content");
    }
}