        if let Ok(hdr) = std::str::from_utf8(header.as_bytes()) {
            hdr == self.allowed_origin
            false
