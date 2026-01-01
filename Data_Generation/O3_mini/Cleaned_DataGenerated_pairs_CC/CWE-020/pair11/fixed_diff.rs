        if len > self.buffer.len() {
            return Err("Invalid length: exceeds buffer size");
        let s = std::str::from_utf8(&self.buffer[..len]).map_err(|_| "UTF8 conversion error")?;
        Ok(s.to_string())
