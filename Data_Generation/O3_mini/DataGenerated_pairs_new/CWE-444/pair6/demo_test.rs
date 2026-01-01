/////////////////////////////////////////////////////////////
// Test Oracle Function for HTTP Request Parsing Consistency  //
/////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inconsistent_request() {
        // This input contains duplicate "Content-Length" headers.
        // The first header indicates a length of 10 while the second indicates 5.
        let input = "GET / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 5\r\n\r\nHelloWorld";
        // For the vulnerable version, parse_input would process the second header value and return a body trimmed to "Hello",
        // which is not the intended full request ("HelloWorld").
        // In the corrected version, the parser rejects the ambiguous input.
        match parse_input(input) {
            Ok(parsed) => {
                // Vulnerable implementation would incorrectly yield a truncated body.
                // Check if the body is exactly "HelloWorld". If not, the vulnerability is exposed.
                assert_eq!(parsed.body, "HelloWorld", "Inconsistent parsing: Expected full body, but got a truncated version.");
            }
            Err(err) => {
                // For the corrected version, the error is expected due to duplicate Content-Length.
                assert_eq!(err, "Duplicate Content-Length header", "Expected rejection of ambiguous input.");
            }
        }
    }
}