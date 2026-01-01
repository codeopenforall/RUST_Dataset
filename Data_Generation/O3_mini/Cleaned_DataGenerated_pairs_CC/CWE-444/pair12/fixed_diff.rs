    fn parse_headers(input: &str) -> HashMap<String, String> {
                headers.insert(key.to_string(), value.to_string());
        let headers = Self::parse_headers(header_part);
        let body = if headers.get("Transfer-Encoding")
                        .map(|v| v.to_lowercase() == "chunked")
                        .unwrap_or(false) {
            Request::decode_chunked(body_part)
        } else if let Some(cl_val) = headers.get("Content-Length") {
                    let line_bytes = line.as_bytes().len();
                    read_bytes += line_bytes;
