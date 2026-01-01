#[cfg(test)]
mod tests {
    use super::process_input;
    #[test]
    fn xss_oracle() {
        let malicious = "<script>alert(1)</script>";
        let expected = "<html><body>Hello, &lt;script&gt;alert(1)&lt;/script&gt;</body></html>";
        let response = process_input(malicious);
        assert_eq!(response, expected, "Output did not escape malicious input correctly; potential XSS vulnerability detected.");
    }
}
