////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Oracle:
// This test function constructs a crafted HTTP request featuring both a Content-Length header (set to 5)
// and a Transfer-Encoding header (chunked). In the vulnerable implementation, only 5 bytes of the body
// are processed, whereas the corrected version decodes the complete chunked body ("PayloadData", 11 bytes).
// The test asserts that the processed body length is exactly 11, ensuring consistent interpretation.
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the Request structure from the current module.
    use super::Request;
    
    #[test]
    fn test_inconsistent_http_parsing() {
        let input = "POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 5\r\nTransfer-Encoding: chunked\r\n\r\n7\r\nPayloadData\r\n0\r\n\r\n";
        if let Some(req) = Request::parse(input) {
            // Expected body is "PayloadData" with length 11.
            assert_eq!(req.body.len(), 11, "HTTP body length mismatch detected");
        } else {
            panic!("Failed to parse HTTP request");
        }
    }
}