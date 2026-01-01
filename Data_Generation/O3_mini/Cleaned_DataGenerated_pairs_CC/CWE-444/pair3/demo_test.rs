//////////////////////////
// Test Oracle Function
//////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_request_smuggling() {
        // Crafted HTTP request with consistent Content-Length header
        // The intended body "Hello World" is 11 bytes long.
        let request_data = b"GET / HTTP/1.1\r\nContent-Length: 11\r\n\r\nHello World";
        
        // Parse the request using the provided functions.
        let req = Request::new(request_data).expect("Parsing failed");
        let outcome = process(&req);
        
        // For the vulnerable version, due to an off-by-one error,
        // the body length is misinterpreted and request validation fails.
        // For the corrected version, it should pass.
        assert_eq!(outcome, Ok("Request processed"), "Request processing failed");
    }
}