fn escape_html(input: &str) -> String {
    input.chars().map(|c| {
        match c {
            '<' => "&lt;".to_owned(),
            '>' => "&gt;".to_owned(),
            '&' => "&amp;".to_owned(),
            '"' => "&quot;".to_owned(),
            '\'' => "&#x27;".to_owned(),
            _ => c.to_string()
        }
    }).collect()
}
        let safe_input = escape_html(input);
        format!("<html><body>Hello, {}!</body></html>", safe_input)
