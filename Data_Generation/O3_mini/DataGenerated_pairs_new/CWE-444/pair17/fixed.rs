////////////////////////////////////////////////////////////
// Corrected Code - Secure HTTP Parsing Implementation
//
// In this version, if the "Transfer-Encoding" header is present and 
// specifies "chunked", then the body is parsed using proper chunked 
// encoding rules. This ensures that the request body is interpreted 
// consistently between different components and mitigates the request 
// smuggling vulnerability.
////////////////////////////////////////////////////////////
use std::collections::HashMap;
use std::thread;

struct Request {
    method: String,
    uri: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

fn parse_request(input: &str) -> Request {
    // Split request into header and body sections.
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    let header_str = parts.get(0).unwrap_or(&"");
    let mut headers = HashMap::new();
    let mut lines = header_str.lines();
    let request_line = lines.next().unwrap_or("");
    let req_parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = req_parts.get(0).unwrap_or(&"").to_string();
    let uri = req_parts.get(1).unwrap_or(&"").to_string();

    for line in lines {
        if let Some((k, v)) = line.split_once(":") {
            headers.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    // Properly parse the body based on the presence of the Transfer-Encoding header.
    let body = if headers
        .get("Transfer-Encoding")
        .map(|v| v.to_ascii_lowercase())
        == Some("chunked".to_string())
    {
        parse_chunked_body(parts.get(1).unwrap_or(&""))
    } else if let Some(cl) = headers.get("Content-Length") {
        let cl_val = cl.parse::<usize>().unwrap_or(0);
        let b = parts.get(1).unwrap_or(&"").as_bytes();
        b[..cl_val.min(b.len())].to_vec()
    } else {
        parts.get(1).unwrap_or(&"").as_bytes().to_vec()
    };

    Request { method, uri, headers, body }
}

// Parses a body encoded with HTTP chunked transfer encoding.
fn parse_chunked_body(chunked: &str) -> Vec<u8> {
    let mut body = Vec::new();
    let mut lines = chunked.lines();
    loop {
        // Each chunk begins with its size in hex.
        let line = match lines.next() {
            Some(l) => l.trim(),
            None => break,
        };
        let chunk_size = usize::from_str_radix(line, 16).unwrap_or(0);
        if chunk_size == 0 {
            // End of chunks.
            break;
        }
        let mut remaining = chunk_size;
        while remaining > 0 {
            if let Some(data_line) = lines.next() {
                let data_bytes = data_line.as_bytes();
                let take = remaining.min(data_bytes.len());
                body.extend_from_slice(&data_bytes[..take]);
                remaining -= take;
            } else {
                break;
            }
        }
    }
    body
}

fn process_input(input: &str) -> Vec<u8> {
    let req = parse_request(input);
    req.body
}

fn main() {
    // Simulated HTTP request that includes both a Content-Length 
    // and a Transfer-Encoding header.
    let input = "POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 10\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nHello\r\n0\r\n\r\nExtra";
    let t = thread::spawn(move || {
        let result = process_input(input);
        println!("Processed body: {:?}", String::from_utf8_lossy(&result));
    });
    t.join().unwrap();
}