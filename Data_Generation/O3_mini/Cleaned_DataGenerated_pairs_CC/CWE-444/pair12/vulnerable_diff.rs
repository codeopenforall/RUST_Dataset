    unsafe fn parse_headers(input: &str) -> HashMap<String, String> {
                let k = String::from_utf8_unchecked(key.as_bytes().to_vec());
                let v = String::from_utf8_unchecked(value.as_bytes().to_vec());
                headers.insert(k, v);
        let headers = unsafe { Self::parse_headers(header_part) };
        let body = if let Some(cl_val) = headers.get("Content-Length") {
        } else if headers.get("Transfer-Encoding")
                        .map(|v| v.to_lowercase() == "chunked")
                        .unwrap_or(false) {
            Request::decode_chunked(body_part)
                    let len = line.len();
                    read_bytes += len;
