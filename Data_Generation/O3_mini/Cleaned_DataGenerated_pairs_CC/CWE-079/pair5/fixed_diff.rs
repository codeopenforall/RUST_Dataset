    fn escape_html(&self, input: &str) -> String {
        input.chars()
            .map(|c| match c {
                '<' => "&lt;".to_string(),
                '>' => "&gt;".to_string(),
                '&' => "&amp;".to_string(),
                '"' => "&quot;".to_string(),
                '\''=> "&#x27;".to_string(),
                _ => c.to_string(),
            })
            .collect()
    }
        let safe_input = self.escape_html(input);
        page.push_str(&format!("<div>{}</div>", safe_input));
