        let sanitized: String = value.chars()
            .filter(|&c| c != '\r' && c != '\n')
            .collect();
        let mut combined = String::new();
        combined.push_str(key);
        combined.push_str(": ");
        combined.push_str(&sanitized);
        resp.header = combined;
            println!("Logged header: {}", resp_clone);
