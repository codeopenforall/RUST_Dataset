use std::fmt::Write;
    header: String,
    footer: String,
            header: "<html><body>".to_string(),
            footer: "</body></html>".to_string(),
    fn escape(input: &str) -> String {
        let mut escaped = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '<' => escaped.push_str("&lt;"),
                '>' => escaped.push_str("&gt;"),
                '&' => escaped.push_str("&amp;"),
                '"' => escaped.push_str("&quot;"),
                '\'' => escaped.push_str("&#x27;"),
                '/' => escaped.push_str("&#x2F;"),
                _ => escaped.push(c),
            }
        escaped
    }
    pub fn render(&self, content: &str) -> String {
        let mut out = String::with_capacity(self.header.len() + self.footer.len() + content.len());
        out.push_str(&self.header);
        let safe_content = Self::escape(content);
        out.push_str(&safe_content);
        out.push_str(&self.footer);
        out
