    let mut lines_iter = raw.split("\r\n");
    let request_line = lines_iter.next()?;
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
    req.method = parts[0].to_string();
    req.uri = parts[1].to_string();
    for line in &mut lines_iter {
        if line.is_empty() {
            break;
        if let Some(pos) = line.find(":") {
            let key = line[..pos].trim().to_string();
            let value = line[pos + 1..].trim().to_string();
            header_map.insert(key, value);
    req.headers = header_map;
    if req.headers.contains_key("Content-Length") && req.headers.contains_key("Transfer-Encoding") {
        return None; 
    }
    req.body = lines_iter.collect::<Vec<&str>>().join("\r\n");
    let benign = "POST / HTTP/1.1\r\nHost: secure\r\nContent-Length: 13\r\n\r\nGET /admin HTTP/1.1\r\n";
    if let Some(req) = parse_http(benign) {
        assert!(!req.body.contains("GET /admin"), "Unexpected request smuggling detected!");
        panic!("Parsing failed on benign input");
