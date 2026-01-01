//////////////////////////////////////////////////////
// Test oracle for detecting improper handling of
// duplicate Content-Length headers. This test should
// fail when the vulnerable implementation is executed,
// and pass with the corrected version.
//////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn oracle() {
        let input = b"POST / HTTP/1.1\r\nContent-Length: 10\r\nContent-Length: 20\r\n\r\n0123456789ABCDEFGHIJKLMNOPQRST";
        
        // For the unsafe (vulnerable) parser, an unsafe block is required.
        // In the vulnerable implementation, the duplicate header is silently overridden,
        // resulting in a successful parse of the wrong body length.
        // Thus, the test asserts that an error is expected.
        unsafe {
            let result = Request::parse_http(input);
            assert!(result.is_err(), "Expected error due to duplicate Content-Length header, but got Ok");
        }
    }
}