use std::thread;
struct Renderer;
impl Renderer {
    pub unsafe fn build_html(&self, input: &str) -> String {
        let mut html = String::from("<html><body>User: ");
        let ptr = input.as_ptr();
        let len = input.len();
        let raw_slice = std::slice::from_raw_parts(ptr, len);
        let untrusted = std::str::from_utf8_unchecked(raw_slice);
        html.push_str(untrusted);
        html.push_str("</body></html>");
        html
    }
}
fn process_input(input: &str) -> String {
    let renderer = Renderer;
    let inp = input.to_owned();
    let handle = thread::spawn(move || unsafe { renderer.build_html(&inp) });
    handle.join().unwrap()
}
fn main() {
    let malicious = "<script>alert('XSS');</script>";
    let output = process_input(malicious);
    println!("{}", output);
}
