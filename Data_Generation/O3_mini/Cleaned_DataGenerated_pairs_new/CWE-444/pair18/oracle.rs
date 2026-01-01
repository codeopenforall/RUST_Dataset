#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let input = "GET / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n";
        let req = parse_request(input);
        assert_eq!(req.length, 10, "Computed length should be 10");
    }
}
