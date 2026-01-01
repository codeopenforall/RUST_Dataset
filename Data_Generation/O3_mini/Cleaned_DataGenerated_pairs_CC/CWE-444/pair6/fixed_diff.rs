    let mut header_lines = header_part.lines();
    let request_line = header_lines.next().ok_or("Missing request line")?;
    for line in header_lines {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            continue;
        let key = parts[0].trim().to_string();
        let value = parts[1].trim().to_string();
        if headers.contains_key(&key) {
            if key == "Content-Length" {
                return Err("Duplicate Content-Length header");
            }
        }
        headers.insert(key, value);
        let expected = cl.parse::<usize>().map_err(|_| "Invalid Content-Length value")?;
        if expected != body_part.len() {
            return Err("Message body length does not match Content-Length header");
