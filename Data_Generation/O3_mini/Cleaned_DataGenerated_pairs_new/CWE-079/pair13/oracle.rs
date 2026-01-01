#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
        let engine = Renderer::new();
        let malicious = "<script>alert('xss')</script>";
        let output = engine.render(malicious);
        assert!(
            !output.contains("<script>"),
            "Test failed: Unescaped script tag found in output."
        );
    }
}
