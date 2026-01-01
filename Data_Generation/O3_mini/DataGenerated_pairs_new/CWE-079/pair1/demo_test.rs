#[cfg(test)]
mod tests {
    // Import the public API from the compiled unit.
    // For the vulnerable build, this test should fail,
    // because the output will reflect unescaped input.
    // For the fixed build, the test will pass.
    use super::process_input;

    #[test]
    fn xss_oracle() {
        let malicious = "<script>alert(1)</script>";
        let expected = "<html><body>Hello, &lt;script&gt;alert(1)&lt;/script&gt;</body></html>";
        let response = process_input(malicious);
        // The test contracts that the safe version does NOT reflect raw HTML.
        // In case of vulnerability, the response would contain the unescaped malicious tag.
        assert_eq!(response, expected, "Output did not escape malicious input correctly; potential XSS vulnerability detected.");
    }
}