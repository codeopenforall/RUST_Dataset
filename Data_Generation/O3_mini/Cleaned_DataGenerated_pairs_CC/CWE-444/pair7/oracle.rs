#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_request_processing() {
        let input = "Content-Length: 20\n\n12345678901234567890";
        let req = Request::new(input.to_string());
        let result = req.process();
        assert!(result.is_ok(), "Processing should succeed when header and body lengths match");
    }
}
