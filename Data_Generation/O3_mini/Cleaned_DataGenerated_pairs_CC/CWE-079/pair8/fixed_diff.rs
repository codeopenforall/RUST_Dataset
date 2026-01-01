    fn render(&self, user_input: &str) -> String {
        let safe_input = escape_html(user_input);
        self.content.replace("{content}", &safe_input)
}
fn escape_html(input: &str) -> String {
    input.chars().map(|c| match c {
        '<' => "&lt;".to_string(),
        '>' => "&gt;".to_string(),
        '&' => "&amp;".to_string(),
        '"' => "&quot;".to_string(),
        '\'' => "&#x27;".to_string(),
        _ => c.to_string(),
    }).collect()
        let page = template_clone.render(&user_clone);
