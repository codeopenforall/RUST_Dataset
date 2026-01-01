#[cfg(test)]
mod tests {
    use super::Page;
    #[test]
    fn verify_protection() {
        let malicious = "<script>alert('xss');</script>";
        let page = Page::new("<html><body>");
        let output = page.assemble(malicious);
        assert!(!output.contains("<script>"), "Output contains unsanitized script tag");
        assert!(output.contains("&lt;script&gt;"), "Output does not contain properly escaped content");
    }
}
