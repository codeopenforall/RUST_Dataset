#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_request_smuggling() {
        let request_data = b"GET / HTTP/1.1\r\nContent-Length: 11\r\n\r\nHello World";
        let req = Request::new(request_data).expect("Parsing failed");
        let outcome = process(&req);
        assert_eq!(outcome, Ok("Request processed"), "Request processing failed");
    }
}
