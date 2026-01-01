#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_html_safety() {
        let malicious_input = "<script>alert('XSS')</script>";
        let response = process_request(malicious_input);
        assert!(
            !response.contains("<script>"),
            "Response contains unsanitized script tag!"
        );
    }
}
