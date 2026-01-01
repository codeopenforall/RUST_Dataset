struct Renderer {
impl Renderer {
    fn render(&self, input: &str) -> String {
            result.push_str(s);
    fn start(&self, input: String) -> String {
            *guard = format!("<html><body>{}</body></html>", input);
    let renderer = Renderer::new();
    renderer.start(renderer.render(input))
