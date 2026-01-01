#[cfg(test)]
mod tests {
    // The test oracle validates the correctness of the HTTP request parser.
    // It uses an input where the Content-Length header overstates the amount of available body data.
    // The expected behavior is that the parser only reads the available data ("BODY") and ignores any extra bytes.
    // For the uncorrected (vulnerable) version, this test should fail, while for the corrected version it should pass.
    use super::HttpRequest;

    #[test]
    fn test_http_request_parsing() {
        let input = "GET / HTTP/1.1\r\nContent-Length: 50\r\nHost: example.com\r\n\r\nBODY";
        // Using an unsafe block if necessary; it works with both safe and unsafe implementations.
        #[allow(unused_unsafe)]
        let req = unsafe { HttpRequest::parse(input) };
        // The expected body is only "BODY" because only those bytes are available after the headers.
        assert_eq!(req.body, "BODY", "The parsed body does not match the expected output.");
    }
}