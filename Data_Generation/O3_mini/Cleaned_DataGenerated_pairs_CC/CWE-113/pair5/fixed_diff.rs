        let sanitized: String = input.chars()
            .filter(|&c| c != '\r' && c != '\n')
            .collect();
        let header = format!("HTTP/1.1 200 OK\r\nCustom-Header: {}\r\n", sanitized);
