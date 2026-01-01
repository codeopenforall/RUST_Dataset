        let sanitized: String = value.chars().filter(|&c| c != '\r' && c != '\n').collect();
        let header_line = format!("{}: {}", key, sanitized);
        self.headers.push(header_line);
