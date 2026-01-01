#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let malicious = "GET / HTTP/1.1\r\nContent-Length: 0\r\nX-Extra: exploit\r\n\r\nBODY";
        let req = Request { raw: malicious.to_owned() };
        assert!(!process_request(&req), "Malicious request should be rejected");
    }
}
