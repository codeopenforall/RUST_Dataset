#[cfg(test)]
mod tests {
    use super::parse_http;
    #[test]
    fn test_smuggling_detection() {
        let request = "POST / HTTP/1.1\r\nHost: test\r\nContent-Length: 13\r\nTransfer-Encoding: chunked\r\n\r\nGET /admin HTTP/1.1\r\n";
        let result = parse_http(request);
        assert!(result.is_none(), "Ambiguous request was not rejected, potential request smuggling!");
    }
}
