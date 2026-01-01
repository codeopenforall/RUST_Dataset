#[cfg(test)]
mod tests {
    use std::panic;
    use crate::parse_http;
    #[test]
    fn test_request_smuggling() {
        let request = "POST /submit HTTP/1.1\r\nHost: example.com\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n12345678901234567890";
        let result = panic::catch_unwind(|| {
            parse_http(request).unwrap();
        });
        assert!(result.is_err(), "Request smuggling vulnerability was not triggered as expected!");
    }
}
