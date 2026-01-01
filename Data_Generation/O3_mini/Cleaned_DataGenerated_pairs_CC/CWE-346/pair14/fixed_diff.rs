        if let Some(fwd) = headers.get("X-Forwarded-Host") {
            if fwd != self.trusted {
                return false;
    let mut headers = std::collections::HashMap::new();
