//////////////////////////////
// Insecure HTTP parser example
//////////////////////////////
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
        // Check for Content-Length header case-insensitively.
        if line.to_lowercase().starts_with("content-length:") {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                if let Ok(val) = parts[1].trim().parse::<usize>() {
                    len_values.push(val);
                }
            }
        }
    }
    // Insecure handling: if there are multiple Content-Length headers,
    // the values are added together using unsafe pointer arithmetic.
    let computed = if len_values.len() > 1 {
        unsafe {
            let base = len_values.as_ptr();
            let first = *base;
            let second = *base.add(1);
            // Erroneously sum the headers leading to an incorrect total.
            first + second
        }
    } else {
        *len_values.get(0).unwrap_or(&0)
    };
    Request::new(computed)
}

fn main() {
    // Example input with duplicate Content-Length headers.
    let input = "GET / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n";
    let req = parse_request(input);
    println!("Computed length: {}", req.length);
}