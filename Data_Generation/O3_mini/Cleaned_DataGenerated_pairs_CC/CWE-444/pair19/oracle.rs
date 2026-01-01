#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_http_request_parsing() {
        let input = "POST /test HTTP/1.1\r\nTransfer-Encoding: chunked\r\nContent-Length: 5\r\n\r\nHelloWorld";
        let result = process_request(input).expect("Processing failed");
        assert_eq!(result, "HelloWorld", "HTTP request parsing did not prioritize headers correctly");
    }
}
