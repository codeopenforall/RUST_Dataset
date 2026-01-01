    fn sanitize(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }
        let safe_value = Responder::sanitize(value);
        let header_line = format!("{}: {}\r\n", key, safe_value);
