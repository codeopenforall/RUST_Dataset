#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_xss_prevention() {
        let app = App::new();
        let attack_payload = "<script>alert('XSS');</script>".to_string();
        let output = app.handle(attack_payload.clone());
        if !output.contains("&lt;script&gt;") {
            panic!("Test failed: Unsanitized script tag detected in the rendered output.");
        }
        assert!(!output.contains("<script>"), "Raw script tag should not be present in the output.");
    }
}
