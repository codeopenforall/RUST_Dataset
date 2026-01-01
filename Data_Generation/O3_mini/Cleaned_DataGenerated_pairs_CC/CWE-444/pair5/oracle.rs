#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let input = "POST / HTTP/1.1\r\nContent-Length: 30\r\nTransfer-Encoding: chunked\r\n\r\nThis is the request body";
        let req = parse_req(input);
        let expected = req.body.len();
        let result = unsafe { req.process() };
        assert_eq!(result.len(), expected, "The processed body length must equal the actual body length");
    }
}
