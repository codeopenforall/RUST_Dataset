#[cfg(test)]
mod tests {
    use super::*;
    
    // The oracle function verifies that the processed body exactly equals "Hello".
    // In the vulnerable version the output would incorrectly include extra or malformed data.
    #[test]
    fn test_http_parsing() {
        // Input with both conflicting headers.
        let input = "POST / HTTP/1.1\r\nHost: example.com\r\nContent-Length: 10\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nHello\r\n0\r\n\r\nExtra";
        let result = process_input(input);
        // Expected output is exactly the chunked content: "Hello".
        assert_eq!(result, b"Hello".to_vec(), "HTTP parsing did not yield the expected body");
    }
}