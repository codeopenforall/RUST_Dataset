struct Renderer {
impl Renderer {
        Renderer { template: "<html><body>{content}</body></html>".to_string() }
        let safe_input = escape_html(input);
        self.template.replace("{content}", &safe_input)
fn escape_html(input: &str) -> String {
    input.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&#x27;")
}
    let renderer = Renderer::new();
