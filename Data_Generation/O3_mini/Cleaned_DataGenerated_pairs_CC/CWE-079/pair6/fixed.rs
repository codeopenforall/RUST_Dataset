use std::thread;
fn escape_html(input: &str) -> String {
    input.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('\"', "&quot;")
         .replace('\'', "&#x27;")
}
struct Renderer;
impl Renderer {
    pub fn build_html(&self, input: &str) -> String {
        let safe_input = escape_html(input);
        let mut html = String::from("<html><body>User: ");
        html.push_str(&safe_input);
        html.push_str("</body></html>");
        html
    }
}
fn process_input(input: &str) -> String {
    let renderer = Renderer;
    let inp = input.to_owned();
    let handle = thread::spawn(move || renderer.build_html(&inp));
    handle.join().unwrap()
}
fn main() {
    let malicious = "<script>alert('XSS');</script>";
    let output = process_input(malicious);
    println!("{}", output);
}
