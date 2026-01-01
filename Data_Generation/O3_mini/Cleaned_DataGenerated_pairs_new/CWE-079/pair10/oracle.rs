#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_template_rendering() {
        let instance = Renderer::new();
        let malicious_input = "<script>alert('XSS');</script>";
        let output = instance.render(malicious_input);
        assert!(
            !output.contains("<script>"),
            "Output contains raw script tags, potential XSS vulnerability."
        );
        assert!(
            output.contains("&lt;script&gt;") || output.contains("&lt;script"),
            "Output does not properly escape script tags."
        );
    }
}
