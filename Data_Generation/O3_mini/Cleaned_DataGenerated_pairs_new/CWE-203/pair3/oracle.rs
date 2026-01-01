#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generic_error() {
        let svc = AuthService::new();
        let result = svc.verify("charlie", "chocolate");
        assert_eq!(
            result.unwrap_err(),
            "Authentication failed",
            "Error message should be generic to prevent sensitive state disclosure."
        );
    }
}
