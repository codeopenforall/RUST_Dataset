    fn parse(input: &str) -> Self {
        let declared_length: usize = headers.get("Content-Length")
        let available = input.len().saturating_sub(body_offset);
        let actual_length = if declared_length > available { available } else { declared_length };
        let body_bytes = &input.as_bytes()[body_offset..body_offset + actual_length];
        let body = String::from_utf8_lossy(body_bytes).into_owned();
    let req = HttpRequest::parse(request_str);
