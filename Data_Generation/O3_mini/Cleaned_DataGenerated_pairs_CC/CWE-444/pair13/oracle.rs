#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    #[test]
    fn oracle() {
        let input = b"POST / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n0123456789ABCDEFGHIJKLMNOPQRST";
        unsafe {
            let result = Request::parse_http(input);
            assert!(result.is_err(), "Expected error due to duplicate Content-Length header, but got Ok");
        }
    }
}
