/////////////////////////////
// Test Oracle Function
/////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    // This test constructs an HTTP request with a malicious "Origin" header.
    // The contract is that the validation should fail (return false).
    // The test will fail against the vulnerable code (which erroneously returns true)
    // and pass against the corrected version.
    #[test]
    fn test_origin_validation() {
        let mut headers = HashMap::new();
        headers.insert("Origin".to_string(), "https://malicious.example.com".to_string());
        let req = HttpRequest::new(headers);
        let shared_req = Arc::new(Mutex::new(req));
        let result = {
            // Directly using the validate() method in a single-threaded context
            let locked_req = shared_req.lock().unwrap();
            locked_req.validate()
        };
        // The correct behavior is to reject the malicious header (i.e., result must be false).
        assert!(!result, "Origin validation should fail for a malicious header.");
    }
}