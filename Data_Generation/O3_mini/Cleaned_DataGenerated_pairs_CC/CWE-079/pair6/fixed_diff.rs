fn escape_html(input: &str) -> String {
    input.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('\"', "&quot;")
         .replace('\'', "&#x27;")
}
    pub fn build_html(&self, input: &str) -> String {
        let safe_input = escape_html(input);
        html.push_str(&safe_input);
    let handle = thread::spawn(move || renderer.build_html(&inp));
