        let available = self.bytes.len().saturating_sub(offset);
        let valid_count = if count > available { available } else { count };
            let ptr = self.bytes.as_ptr().add(offset);
            let slice = std::slice::from_raw_parts(ptr, valid_count);
            std::str::from_utf8_unchecked(slice).to_string()
    let user_count = 10; 
