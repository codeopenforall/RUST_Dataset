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
        parts.get(1).unwrap_or(&"").as_bytes().to_vec()
}
fn parse_chunked_body(chunked: &str) -> Vec<u8> {
    let mut body = Vec::new();
    let mut lines = chunked.lines();
    loop {
        let line = match lines.next() {
            Some(l) => l.trim(),
            None => break,
        };
        let chunk_size = usize::from_str_radix(line, 16).unwrap_or(0);
        if chunk_size == 0 {
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
