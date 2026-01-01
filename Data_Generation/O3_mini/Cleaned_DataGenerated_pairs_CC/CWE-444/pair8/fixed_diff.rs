            request.headers.push((key.to_string(), value.to_string()));
            if let Ok(cl) = value.trim().parse::<usize>() {
                content_length = Some(cl);
            }
         return Err("Ambiguous request with both Transfer-Encoding and Content-Length headers");
        if body_start.checked_add(len).filter(|&end| end <= req.len()).is_none() {
        let body_slice = &req.as_bytes()[body_start..body_start+len];
        request.body = body_slice.to_vec();
    } else if transfer_encoding {
        let body_start = req.find("\r\n\r\n").ok_or("Invalid request")? + 4;
        request.body = req.as_bytes()[body_start..].to_vec();
                    match process_request(&parsed) {
                        Ok(result) => println!("Processed sum: {}", result),
                        Err(e) => println!("Processing error: {}", e),
                    }
                Err(e) => println!("Parsing error: {}", e),
