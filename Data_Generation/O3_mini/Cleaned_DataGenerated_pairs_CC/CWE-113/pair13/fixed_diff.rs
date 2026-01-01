    fn sanitize(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }
        let safe_input = Self::sanitize(input);
        let header = format!("{}{}{}", base, safe_input, "\r\nContent-Length: 0\r\n\r\n");
