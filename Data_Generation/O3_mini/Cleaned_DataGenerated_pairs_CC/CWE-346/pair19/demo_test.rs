#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Test function for validating the origin check.
    // In the vulnerable version, using a header value that exactly matches
    // "https://valid.example.com" will fail because pointer comparison does not hold.
    // In the corrected version, the check will succeed.
    #[test]
    fn test_origin_validation() {
        // Construct a request with the trusted header value.
        let req = Request {
            headers: {
                let mut map = HashMap::new();
                map.insert(String::from("Origin"), String::from("https://valid.example.com"));
                map
            },
        };
        let valid = validate(&req);
        assert!(valid, "The origin check did not pass; vulnerability or fix misapplied.");
    }
}