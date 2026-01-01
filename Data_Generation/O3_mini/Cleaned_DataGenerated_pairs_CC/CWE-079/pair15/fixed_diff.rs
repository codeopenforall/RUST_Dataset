fn escape_html(input: &str) -> String {
    input.chars().map(|c| {
        match c {
            '&' => "&amp;".to_owned(),
            '<' => "&lt;".to_owned(),
            '>' => "&gt;".to_owned(),
            '"' => "&quot;".to_owned(),
            '\'' => "&#x27;".to_owned(),
            '/' => "&#x2F;".to_owned(),
            _ => c.to_string(),
        }
    }).collect::<String>()
}
        let safe_input = escape_html(input);
        self.tpl.replace("{user_data}", &safe_input)
