#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_inconsistent_request() {
        let input = "GET / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 5\r\n\r\nHelloWorld";
        match parse_input(input) {
            Ok(parsed) => {
                assert_eq!(parsed.body, "HelloWorld", "Inconsistent parsing: Expected full body, but got a truncated version.");
            }
            Err(err) => {
                assert_eq!(err, "Duplicate Content-Length header", "Expected rejection of ambiguous input.");
            }
        }
    }
}
