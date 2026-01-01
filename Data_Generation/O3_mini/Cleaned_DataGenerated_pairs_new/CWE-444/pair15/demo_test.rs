#[cfg(test)]
mod tests {
    use std::panic;
    // Assume the library exposes the following public function for parsing:
    // pub fn parse_http(request: &str) -> Result<HttpRequest, &'static str>
    // When testing the subject crate, this function will either panic (if vulnerable)
    // or return Ok (if fixed) when given a malicious request.

    use crate::parse_http;

    #[test]
    fn test_request_smuggling() {
        // Crafted HTTP request with duplicate Content-Length headers to simulate header smuggling.
        let request = "POST /submit HTTP/1.1\r\nHost: example.com\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n12345678901234567890";
        let result = panic::catch_unwind(|| {
            // This call should panic in the vulnerable implementation because the parsed body length
            // will not match the unsafely stored global Content-Length.
            parse_http(request).unwrap();
        });
        assert!(result.is_err(), "Request smuggling vulnerability was not triggered as expected!");
    }
}