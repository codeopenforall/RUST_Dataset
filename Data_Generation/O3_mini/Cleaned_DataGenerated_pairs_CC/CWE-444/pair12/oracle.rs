#[cfg(test)]
mod tests {
    use super::Request;
    #[test]
    fn test_inconsistent_http_parsing() {
        let input = "POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 5\r\nTransfer-Encoding: chunked\r\n\r\n7\r\nPayloadData\r\n0\r\n\r\n";
        if let Some(req) = Request::parse(input) {
            assert_eq!(req.body.len(), 11, "HTTP body length mismatch detected");
        } else {
            panic!("Failed to parse HTTP request");
        }
    }
}
