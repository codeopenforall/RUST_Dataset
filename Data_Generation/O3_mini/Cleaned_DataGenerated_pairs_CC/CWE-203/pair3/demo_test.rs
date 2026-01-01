#[cfg(test)]
mod tests {
    use super::*;

    // This test serves as an oracle verifying that error messages are generic.
    // For the vulnerable variant, the error would leak the user existence; for the fixed variant, it should always be "Authentication failed".
    #[test]
    fn test_generic_error() {
        let svc = AuthService::new();
        // Trigger an authentication with a non-existent user.
        let result = svc.verify("charlie", "chocolate");
        assert_eq!(
            result.unwrap_err(),
            "Authentication failed",
            "Error message should be generic to prevent sensitive state disclosure."
        );
    }
}