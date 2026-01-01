        let sanitized: String = input.chars().filter(|&c| c != '\r' && c != '\n').collect();
            header: format!("Location: {}", sanitized),
