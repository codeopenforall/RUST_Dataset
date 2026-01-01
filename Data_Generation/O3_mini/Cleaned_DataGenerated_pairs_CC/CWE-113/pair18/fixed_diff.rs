        let sanitized: String = input.chars().filter(|&c| c != '\r' && c != '\n').collect();
        let combined = String::from(base) + &sanitized + "\r\n\r\n";
