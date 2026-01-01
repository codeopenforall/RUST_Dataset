    let request_str = match str::from_utf8(request.as_bytes()) {
        Ok(r) => r,
        Err(_) => return Err("Invalid UTF-8 in request"),
    };
    for line in request_str.split("\r\n") {
        if line.starts_with("Content-Length:") {
            let header_value = line["Content-Length:".len()..].trim();
            if let Ok(value) = header_value.parse::<u64>() {
                if content_length_val.is_some() {
                    return Err("Ambiguous Content-Length header");
                content_length_val = Some(value);
            } else {
                return Err("Invalid Content-Length value");
