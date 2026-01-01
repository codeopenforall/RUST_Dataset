use std::str::FromStr;
struct Request {
    length: usize,
}
impl Request {
    fn new(length: usize) -> Self {
        Self { length }
    }
}
fn parse_request(input: &str) -> Request {
    let mut len_values: Vec<usize> = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            break;
        }
        if line.to_lowercase().starts_with("content-length:") {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                if let Ok(val) = parts[1].trim().parse::<usize>() {
                    len_values.push(val);
                }
            }
        }
    }
    let computed = if !len_values.is_empty() {
        len_values[0]
    } else {
        0
    };
    Request::new(computed)
}
fn main() {
    let input = "GET / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n";
    let req = parse_request(input);
    println!("Computed length: {}", req.length);
}
