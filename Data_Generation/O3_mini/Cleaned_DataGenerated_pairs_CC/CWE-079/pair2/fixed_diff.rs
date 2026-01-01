        output.push_str(&self.header);
        output.push_str(&escape_html(user_content));
        output.push_str(&self.footer);
}
fn escape_html(input: &str) -> String {
    input.chars().map(|c| {
        match c {
            '<'  => "&lt;".to_string(),
            '>'  => "&gt;".to_string(),
            '&'  => "&amp;".to_string(),
            '"'  => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            _    => c.to_string(),
        }
    }).collect::<String>()
