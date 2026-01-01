#[cfg(test)]
mod tests {
    use super::server;
    #[test]
    fn test_request_smuggling() {
        let malicious_req = b"POST / HTTP/1.1\r\nHost: test\r\nContent-Length: 5\r\nContent-Length: 10\r\n\r\nhello world";
        let res = unsafe { server::process_request(malicious_req) };
        assert!(res.is_err(), "Request with duplicate conflicting headers must be rejected");
    }
}
