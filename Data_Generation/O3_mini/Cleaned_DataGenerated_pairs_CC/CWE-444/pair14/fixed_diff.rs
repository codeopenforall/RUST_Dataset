    if req.headers.contains_key("Content-Length") && req.headers.contains_key("Transfer-Encoding") {
        return Err("Ambiguous headers: both Content-Length and Transfer-Encoding are present");
    }
        let clen: usize = cl_val.parse().map_err(|_| "Invalid Content-Length")?;
        req.body = input[header_end..header_end+clen].to_string();
            body.push_str(&input[pos..pos+chunk_size]);
        shared_clone.store(1, Ordering::SeqCst);
