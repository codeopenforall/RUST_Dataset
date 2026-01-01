use std::str;
    let mut lines = header_part.lines();
    let request_line = lines.next().ok_or("Missing request line")?;
    for line in lines {
        unsafe {
            let ptr = line.as_ptr();
            let mut pos = 0;
            while pos < line.len() && *ptr.add(pos) != b':' {
                pos += 1;
            }
            if pos >= line.len() {
                continue;
            }
            let key = String::from_utf8_unchecked(Vec::from(&line.as_bytes()[0..pos]));
            let value = String::from_utf8_unchecked(Vec::from(&line.as_bytes()[pos+1..]));
            headers.insert(key.trim().to_string(), value.trim().to_string());
        if let Ok(n) = cl.parse::<usize>() {
            unsafe {
                let body_ptr = body_part.as_ptr();
                let body_slice = std::slice::from_raw_parts(body_ptr, n);
                let body_str = str::from_utf8_unchecked(body_slice);
                return Ok(HTTPRequest {
                    method: req_parts[0].to_string(),
                    path: req_parts[1].to_string(),
                    headers,
                    body: body_str.to_string(),
                });
            }
