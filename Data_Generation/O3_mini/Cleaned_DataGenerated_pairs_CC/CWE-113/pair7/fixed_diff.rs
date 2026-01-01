        let safe_value: String = value.chars().filter(|&c| c != '\r' && c != '\n').collect();
            let line = format!("{}: {}\r\n", key, safe_value);
