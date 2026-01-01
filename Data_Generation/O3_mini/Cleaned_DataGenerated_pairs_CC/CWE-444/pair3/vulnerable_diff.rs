use std::ptr;
        unsafe {
            let raw_str = str::from_utf8_unchecked(raw);
            let mut headers = Vec::new();
            let mut lines = raw_str.split("\r\n");
            let _ = lines.next()?;
            for line in lines.by_ref() {
                if line.is_empty() {
                    break;
                }
                let delimiter = line.find(':').unwrap_or(0);
                let key = line[..delimiter].to_string();
                let value = line[delimiter + 1..].trim().to_string();
                headers.push((key, value));
            let body_marker = "\r\n\r\n";
            let body_start = raw_str.find(body_marker).map(|p| p + body_marker.len()).unwrap_or(raw.len());
            let body_len = raw.len() - body_start;
            let mut body = Vec::with_capacity(body_len);
            ptr::copy_nonoverlapping(raw.as_ptr().add(body_start), body.as_mut_ptr(), body_len);
            body.set_len(body_len.saturating_sub(1)); 
            Some(Request { headers, body })
            if let Some(exist) = cnt_length {
                if exist != parsed {
    let expected = cnt_length.unwrap_or(req.body.len());
    if expected != req.body.len() {
        return Err("Body length mismatch");
