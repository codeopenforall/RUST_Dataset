        if offset.checked_add(len).map_or(true, |end| end > DATA.len()) {
            return Err("Invalid input: offset and length exceed data bounds".to_string());
        let slice = &DATA[offset..offset+len];
        let s = std::str::from_utf8(slice).map_err(|_| "UTF-8 conversion error".to_string())?;
        Ok(s.to_string())
