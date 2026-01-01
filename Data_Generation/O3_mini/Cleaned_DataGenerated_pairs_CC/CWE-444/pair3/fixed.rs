use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::str;
struct Request {
    headers: Vec<(String, String)>,
    body: Vec<u8>,
}
impl Request {
    fn new(raw: &[u8]) -> Option<Self> {
        let raw_str = std::str::from_utf8(raw).ok()?;
        let mut headers = Vec::new();
        let mut lines = raw_str.split("\r\n");
        let _ = lines.next()?;
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                continue;
            }
            headers.push((parts[0].trim().to_string(), parts[1].trim().to_string()));
        }
        let body_marker = "\r\n\r\n";
        let body_start = raw_str.find(body_marker).map(|p| p + body_marker.len()).unwrap_or(raw.len());
        let body = raw[body_start..].to_vec();
        Some(Request { headers, body })
    }
}
fn process(req: &Request) -> Result<&'static str, &'static str> {
    let mut cnt_length = None;
    for (key, value) in &req.headers {
        if key.eq_ignore_ascii_case("Content-Length") {
            let parsed = value.parse::<usize>().map_err(|_| "Invalid Content-Length")?;
            if let Some(existing) = cnt_length {
                if existing != parsed {
                    return Err("Inconsistent Content-Length headers");
                }
            } else {
                cnt_length = Some(parsed);
            }
        }
    }
    if let Some(expected) = cnt_length {
        if expected != req.body.len() {
            return Err("Body length mismatch");
        }
    }
    Ok("Request processed")
}
fn handle_conn(mut stream: TcpStream) {
    let mut buffer = vec![0; 4096];
    if let Ok(n) = stream.read(&mut buffer) {
        if let Some(req) = Request::new(&buffer[..n]) {
            let resp = process(&req).unwrap_or("Error processing request");
            let _ = stream.write_all(resp.as_bytes());
        }
    }
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(|| {
                handle_conn(stream);
            });
        }
    }
}
