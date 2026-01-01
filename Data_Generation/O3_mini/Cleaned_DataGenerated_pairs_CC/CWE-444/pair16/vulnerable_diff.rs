    unsafe { 
        let ptr = request.as_ptr();
        let len = request.len();
        let raw_slice = std::slice::from_raw_parts(ptr, len);
        let request_str = str::from_utf8_unchecked(raw_slice);
        for line in request_str.split("\r\n") {
            if line.starts_with("Content-Length:") {
                let header_value = line["Content-Length:".len()..].trim();
                if let Ok(value) = header_value.parse::<u64>() {
                    content_length_val = Some(value); 
                } else {
                    return Err("Invalid Content-Length value");
