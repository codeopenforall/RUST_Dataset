/*
Below is the oracle test function. It must be placed in a separate test module or file.
This function tests that processing a malicious request with duplicate headers returns an error
in the secure version, while the insecure version would incorrectly return a valid Content-Length.
*/
#[cfg(test)]
mod tests {
    use super::process_request;

    #[test]
    fn test_oracle() {
        // Malicious HTTP request with duplicate Content-Length headers.
        let malicious_request = "POST / HTTP/1.1\r\n\
                                 Host: example.com\r\n\
                                 Content-Length: 5\r\n\
                                 Content-Length: 10\r\n\
                                 \r\n\
                                 helloWorld";
        let result = process_request(malicious_request);
        // In secure implementation, ambiguous headers should result in an error.
        assert!(result.is_err(), "Request with duplicate Content-Length headers should be rejected");
    }
}