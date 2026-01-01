#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_xss() {
        let malicious = "<script>alert('XSS');</script>";
        let renderer = TemplateRenderer::new();
        renderer.append_user_input(malicious);
        let output = renderer.render();
        if output.contains("&lt;script&gt;") && !output.contains("<script>") {
            assert!(true, "Output is properly escaped.");
        } else {
            panic!("Output is not properly escaped, potential XSS vulnerability detected.");
        }
    }
}
