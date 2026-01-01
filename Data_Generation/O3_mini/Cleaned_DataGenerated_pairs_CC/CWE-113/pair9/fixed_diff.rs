            let sanitized: String = location.chars().filter(|&c| c != '\r' && c != '\n').collect();
            let raw = sanitized.as_ptr();
            let len = sanitized.len();
    let input = "http://example.com";
