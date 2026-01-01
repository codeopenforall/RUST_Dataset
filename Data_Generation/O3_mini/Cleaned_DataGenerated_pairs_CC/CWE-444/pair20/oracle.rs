#[cfg(test)]
mod tests {
    use super::HttpRequest;
    #[test]
    fn test_http_request_parsing() {
        let input = "GET / HTTP/1.1\r\nContent-Length: 50\r\nHost: example.com\r\n\r\nBODY";
        #[allow(unused_unsafe)]
        let req = unsafe { HttpRequest::parse(input) };
        assert_eq!(req.body, "BODY", "The parsed body does not match the expected output.");
    }
}
