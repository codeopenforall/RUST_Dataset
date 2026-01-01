#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ambiguous_request() {
        let req = "POST / HTTP/1.1\r\nTransfer-Encoding: chunked\r\nContent-Length: 11\r\n\r\nHello World";
        match parse_http(req) {
            Ok(_) => panic!("Test failed: ambiguous request should be rejected"),
            Err(e) => assert_eq!(e, "Ambiguous request with both Transfer-Encoding and Content-Length headers"),
        }
    }
}
