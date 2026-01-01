#[cfg(test)]
mod tests {
    // The test oracle expects that ambiguous requests are rejected.
    // In the vulnerable implementation the request will be processed, causing this test to fail.
    // In the corrected implementation the parse function returns an error.
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