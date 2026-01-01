        if expected_len > data.len() {
            return Err("Length exceeds payload size".to_owned());
        let subslice = &data.as_bytes()[..expected_len];
        let result = str::from_utf8(subslice).map_err(|_| "Payload is not valid UTF-8")?;
        Ok(result.to_owned())
