        let safe_value: String = value.chars().filter(|&c| c != '\r' && c != '\n').collect();
        self.headers.push(format!("{}: {}", key, safe_value));
