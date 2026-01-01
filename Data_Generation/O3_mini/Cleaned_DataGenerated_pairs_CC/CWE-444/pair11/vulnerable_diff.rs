    let mut lines = raw.split("\r\n");
    if let Some(request_line) = lines.next() {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }
        req.method = parts[0].to_string();
        req.uri = parts[1].to_string();
    } else {
    let mut reached_body = false;
    unsafe {
        let raw_ptr = raw.as_ptr();
        let raw_len = raw.len();
        let mut index = 0;
        while index < raw_len - 3 {
            let slice = std::slice::from_raw_parts(raw_ptr.add(index), 4);
            if slice == b"\r\n\r\n" {
                reached_body = true;
                break;
            }
            index += 1;
        let header_part = std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, index));
        for line in header_part.lines().skip(1) {
            if let Some(pos) = line.find(":") {
                let key = line[..pos].trim().to_string();
                let value = line[pos+1..].trim().to_string();
                header_map.insert(key, value);
            }
        }
        req.headers = header_map;
        if reached_body {
            if req.headers.contains_key("Content-Length") {
                let cl: usize = req.headers.get("Content-Length").unwrap().parse().unwrap_or(0);
                let body_start = index + 4; 
                if body_start + cl <= raw_len {
                    let body_ptr = raw_ptr.add(body_start);
                    req.body = String::from_utf8_lossy(std::slice::from_raw_parts(body_ptr, cl)).to_string();
                }
            } else {
                let body_slice = std::slice::from_raw_parts(raw_ptr.add(index + 4), raw_len - index - 4);
                req.body = String::from_utf8_lossy(body_slice).to_string();
            }
    let crafted = "POST / HTTP/1.1\r\nHost: vulnerable\r\nContent-Length: 13\r\nTransfer-Encoding: chunked\r\n\r\nGET /admin HTTP/1.1\r\n";
    if let Some(req) = parse_http(crafted) {
        if req.body.contains("GET /admin") {
            panic!("Request smuggling detected!");
        }
        println!("Failed to parse request");
