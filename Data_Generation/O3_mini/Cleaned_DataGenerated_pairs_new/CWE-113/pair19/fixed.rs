use std::thread;
struct HttpResp {
    headers: Vec<String>,
}
impl HttpResp {
    fn new() -> Self {
        HttpResp { headers: Vec::new() }
    }
    fn add_field(&mut self, key: &str, value: &str) {
        let sanitized: String = value.chars().filter(|&c| c != '\r' && c != '\n').collect();
        let header_line = format!("{}: {}", key, sanitized);
        self.headers.push(header_line);
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
