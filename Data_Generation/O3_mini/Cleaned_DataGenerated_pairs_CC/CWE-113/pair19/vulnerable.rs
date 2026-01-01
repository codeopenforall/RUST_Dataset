use std::thread;
struct HttpResp {
    headers: Vec<String>,
}
impl HttpResp {
    fn new() -> Self {
        HttpResp { headers: Vec::new() }
    }
    fn add_field(&mut self, key: &str, value: &str) {
        unsafe {
            let raw = value.as_bytes();
            let untrusted = std::str::from_utf8_unchecked(raw);
            let header_line = format!("{}: {}", key, untrusted);
            self.headers.push(header_line);
        }
    }
    fn get_fields(&self) -> &Vec<String> {
        &self.headers
    }
}
fn construct_response(input: &str) -> HttpResp {
    let mut resp = HttpResp::new();
    let input_clone = input.to_string();
    let handle = thread::spawn(move || {
        resp.add_field("X-Custom", &input_clone);
        resp
    });
    handle.join().unwrap()
}
fn main() {
    let user_input = "safe\r\nInjected: malicious";
    let response = construct_response(user_input);
    for field in response.get_fields() {
        println!("{}", field);
    }
}
