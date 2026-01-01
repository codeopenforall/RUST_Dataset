#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
        let renderer = Renderer::new();
        let input = "<script>alert('XSS');</script>";
        let output = renderer.generate(input);
        let expected = "<html><body>&lt;script&gt;alert(&#x27;XSS&#x27;);&lt;/script&gt;</body></html>";
        assert_eq!(output, expected, "User input was not properly escaped; potential XSS vulnerability remains.");
    }
}
