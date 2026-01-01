#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_http_parsing() {
        let input = "POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 10\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nHello\r\n0\r\n\r\nExtra";
        let result = process_input(input);
        assert_eq!(result, b"Hello".to_vec(), "HTTP parsing did not yield the expected body");
    }
}
