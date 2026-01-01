///////////////////////////
// Test Oracle Function  //
///////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_html_safety() {
        // Malicious payload designed to trigger XSS if not escaped.
        let malicious_input = "<script>alert('XSS')</script>";
        let response = process_request(malicious_input);
        // For a secure implementation, the response should not have the raw "<script>" substring.
        // Instead, it should have escaped equivalents (e.g., "&lt;script&gt;").
        assert!(
            !response.contains("<script>"),
            "Response contains unsanitized script tag!"
        );
    }
}