#[cfg(test)]
mod tests {
    // The test oracle applies the same HTTP input to the parser.
    // In the vulnerable variant, the unsafe offset leads to extracting "elloW" instead of "HelloWorld".
    // The corrected version should consistently return "HelloWorld".
    use super::*;

    #[test]
    fn test_http_request_parsing() {
        let input = "POST /test HTTP/1.1\r\nTransfer-Encoding: chunked\r\nContent-Length: 5\r\n\r\nHelloWorld";
        let result = process_request(input).expect("Processing failed");
        // Expect the full "HelloWorld" as the correct interpretation.
        assert_eq!(result, "HelloWorld", "HTTP request parsing did not prioritize headers correctly");
    }
}