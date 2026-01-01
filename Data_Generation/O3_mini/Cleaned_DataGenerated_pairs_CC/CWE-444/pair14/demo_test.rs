/*
Test Oracle: This test verifies that ambiguous HTTP requests are rejected.
The test should fail when executed against the vulnerable implementation (which accepts ambiguous requests)
and pass when executed against the corrected implementation (which returns an error for ambiguous requests).
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        let ambiguous_request = concat!(
            "POST / HTTP/1.1\r\n",
            "Host: example.com\r\n",
            "Content-Length: 13\r\n",
            "Transfer-Encoding: chunked\r\n",
            "\r\n",
            "Hello, world!"
        );
        let result = process_request(ambiguous_request);
        assert!(result.is_err(), "Ambiguous request must be rejected");
    }
}