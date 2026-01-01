/*
Test Oracle:
This unit test verifies that the processing function succeeds when the request includes a valid
Content-Length header and a corresponding body of the specified length. The test is expected to fail
when run against the vulnerable implementation (due to unsafe memory handling) and pass with the corrected
version.

To run the test, use: cargo test -- --nocapture
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_processing() {
        // This input simulates a valid request where the Content-Length (20) matches the body length.
        let input = "Content-Length: 20\n\n12345678901234567890";
        let req = Request::new(input.to_string());
        let result = req.process();
        // Assert that processing succeeds.
        assert!(result.is_ok(), "Processing should succeed when header and body lengths match");
    }
}