use std::collections::HashSet;
        let mut seen = HashSet::new();
        for line in lines {
            if line.is_empty() {
                break;
            let mut parts = line.splitn(2, ':');
            let key = parts.next().unwrap_or("").trim().to_string();
            let value = parts.next().unwrap_or("").trim().to_string();
            if key.eq_ignore_ascii_case("Content-Length") {
                let lower = key.to_lowercase();
                if !seen.insert(lower) {
                    return Err("Duplicate Content-Length header detected".into());
                }
            }
            headers.push((key, value));
