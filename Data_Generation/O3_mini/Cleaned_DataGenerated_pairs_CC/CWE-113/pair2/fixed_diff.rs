    fn sanitize(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }
        let safe_value = HttpResponse::sanitize(value);
        self.header = format!("{}: {}\r\n", name, safe_value);
