#[cfg(test)]
mod tests {
    use super::*;
    
    // The test oracle uses a hard-coded HTTP request containing duplicate 
    // "Content-Length" headers. The expected behavior is to compute a length 
    // of 10 (i.e. using only the first header). For the insecure version, the 
    // result would erroneously be 30, causing this test to fail.
    #[test]
    fn test_oracle() {
        let input = "GET / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n";
        let req = parse_request(input);
        assert_eq!(req.length, 10, "Computed length should be 10");
    }
}