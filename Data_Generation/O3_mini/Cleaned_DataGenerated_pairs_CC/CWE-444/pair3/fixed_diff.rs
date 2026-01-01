        let raw_str = std::str::from_utf8(raw).ok()?;
        let mut headers = Vec::new();
        let mut lines = raw_str.split("\r\n");
        let _ = lines.next()?;
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                continue;
            }
            headers.push((parts[0].trim().to_string(), parts[1].trim().to_string()));
        let body_marker = "\r\n\r\n";
        let body_start = raw_str.find(body_marker).map(|p| p + body_marker.len()).unwrap_or(raw.len());
        let body = raw[body_start..].to_vec();
        Some(Request { headers, body })
            if let Some(existing) = cnt_length {
                if existing != parsed {
    if let Some(expected) = cnt_length {
        if expected != req.body.len() {
            return Err("Body length mismatch");
        }
