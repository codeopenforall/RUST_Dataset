        let count: usize = header.parse().unwrap();
        let bytes = input.as_bytes();
        unsafe {
            let slice = bytes.get_unchecked(2..2 + count);
            let payload = std::str::from_utf8_unchecked(slice);
            Ok(Config {
                data: payload.to_string(),
            })
    let input = std::env::args().nth(1).unwrap_or_else(|| "10short".to_string());
