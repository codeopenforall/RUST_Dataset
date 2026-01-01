        unsafe {
            let raw = value.as_bytes();
            let untrusted = std::str::from_utf8_unchecked(raw);
            let header_line = format!("{}: {}", key, untrusted);
            self.headers.push(header_line);
        }
