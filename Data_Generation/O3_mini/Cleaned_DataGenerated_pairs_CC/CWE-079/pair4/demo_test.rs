#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_xss() {
        let malicious = "<script>alert('XSS');</script>";
        let renderer = TemplateRenderer::new();
        renderer.append_user_input(malicious);
        let output = renderer.render();
        
        // For the fixed version, the special characters are escaped.
        if output.contains("&lt;script&gt;") && !output.contains("<script>") {
            // Test passes in the fixed code.
            assert!(true, "Output is properly escaped.");
        } else {
            // Test fails in the problematic version.
            panic!("Output is not properly escaped, potential XSS vulnerability detected.");
        }
    }
}