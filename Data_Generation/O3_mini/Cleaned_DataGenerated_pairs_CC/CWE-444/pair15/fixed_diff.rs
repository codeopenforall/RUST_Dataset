use std::sync::Arc;
pub struct ParserStable;
impl HttpParser for ParserStable {
        let mut content_length_value: Option<usize> = None;
                let key = k.trim().to_string();
                let val = v.trim().to_string();
                if key.eq_ignore_ascii_case("Content-Length") {
                    if content_length_value.is_some() {
                        return Err("Multiple Content-Length headers");
                    }
                    content_length_value = Some(val.parse::<usize>().map_err(|_| "Invalid Content-Length")?);
                }
                headers.insert(key, val);
        if let Some(cl) = content_length_value {
            if body_part.len() != cl {
                return Err("Body length does not match Content-Length header");
    ParserStable::parse(request)
    let request = "POST /submit HTTP/1.1\r\nHost: example.com\r\nContent-Length: 10\r\n\r\n1234567890";
            match ParserStable::parse(&req_clone) {
                        assert_eq!(body.len(), 10, "Body length does not match");
