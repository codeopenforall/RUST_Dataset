    fn parse_http(input: &[u8]) -> Result<Self, &'static str> {
        while index < input.len() {
            while line_end < input.len() && input[line_end] != b'\n' {
            let line = &input[index..line_end];
                if key.eq_ignore_ascii_case("Content-Length") && headers.contains_key("Content-Length") {
                    return Err("Duplicate Content-Length header found");
                }
        if index + body_len > input.len() {
        let body = input[index..index + body_len].to_vec();
        match Request::parse_http(raw) {
            Ok(req) => {
                println!("Parsed body: {:?}", str::from_utf8(&req.body).unwrap());
            },
            Err(e) => {
                println!("Error: {}", e);
