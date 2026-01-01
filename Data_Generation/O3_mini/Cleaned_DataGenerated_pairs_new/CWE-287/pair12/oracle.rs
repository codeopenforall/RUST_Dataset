#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authentication() {
        let svc = Service::new();
        let wrong_input = Some("wrongtoken".to_string());
        assert_eq!(svc.authenticate(wrong_input), false, "Authentication should fail for an incorrect token.");
        let correct_input = Some("supersecrettoken".to_string());
        assert_eq!(svc.authenticate(correct_input), true, "Authentication should pass for the correct token.");
    }
}
