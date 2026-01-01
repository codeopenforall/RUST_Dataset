////////////////////////////////////////////////////////////////////////////////////////////////////
// Corrected Code: This version adopts safe parsing routines and properly enforces HTTP semantics.
// When both "Content-Length" and "Transfer-Encoding" are present, the code now prioritizes chunked
// decoding, in accordance with RFC guidelines. This prevents ambiguous interpretation and request smuggling.
////////////////////////////////////////////////////////////////////////////////////////////////////
use std::collections::HashMap;
use std::str;

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    // Safe header parser using standard string methods.
    fn parse_headers(input: &str) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        for line in input.split("\r\n") {
            if let Some(idx) = line.find(':') {
                let key = line[..idx].trim();
                let value = line[idx + 1..].trim();
                headers.insert(key.to_string(), value.to_string());
            }
        }
        headers
    }

    // Corrected parsing: correctly prioritizes the Transfer-Encoding header over Content-Length
    // if both headers are present, and decodes chunked bodies accordingly.
    fn parse(raw: &str) -> Option<Self> {
        let parts: Vec<&str> = raw.split("\r\n\r\n").collect();
        if parts.len() < 2 {
            return None;
        }
        let header_part = parts[0];
        let body_part = parts[1];

        let headers = Self::parse_headers(header_part);

        let body = if headers.get("Transfer-Encoding")
                        .map(|v| v.to_lowercase() == "chunked")
                        .unwrap_or(false) {
            // Prioritize chunked decoding when Transfer-Encoding is present.
            Request::decode_chunked(body_part)
        } else if let Some(cl_val) = headers.get("Content-Length") {
            if let Ok(cl) = cl_val.parse::<usize>() {
                if body_part.len() >= cl {
                    body_part[..cl].to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            body_part.to_string()
        };

        Some(Request { headers, body })
    }

    fn decode_chunked(input: &str) -> String {
        let mut res = String::new();
        let mut lines = input.lines();
        loop {
            let size_line = match lines.next() {
                Some(line) => line,
                None => break,
            };
            let chunk_size = match usize::from_str_radix(size_line, 16) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            let mut chunk = String::new();
            let mut read_bytes = 0;
            while read_bytes < chunk_size {
                if let Some(line) = lines.next() {
                    let line_bytes = line.as_bytes().len();
                    chunk.push_str(line);
                    read_bytes += line_bytes;
                } else {
                    break;
                }
            }
            res.push_str(&chunk);
        }
        res
    }
}

fn main() {
    // The same crafted request now is handled correctly.
    // With Transfer-Encoding present, the code decodes the full chunked body.
    let input = "POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 5\r\nTransfer-Encoding: chunked\r\n\r\n7\r\nPayloadData\r\n0\r\n\r\n";
    if let Some(req) = Request::parse(input) {
        println!("Processed body length: {}", req.body.len());
    } else {
        println!("Failed to parse request");
    }
}