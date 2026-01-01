        let count: usize = header.parse().map_err(|_| "Header parse error")?;
        if input.len() < 2 + count {
            return Err("Payload length mismatch");
        let slice = &input.as_bytes()[2..2 + count];
        let payload = std::str::from_utf8(slice).map_err(|_| "Invalid UTF-8 in payload")?;
        Ok(Config {
            data: payload.to_string(),
        })
    let input = std::env::args().nth(1).unwrap_or_else(|| "05hello".to_string());
