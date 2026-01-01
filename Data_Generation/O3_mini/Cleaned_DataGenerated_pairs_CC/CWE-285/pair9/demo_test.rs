#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorization_oracle() {
        // Initialize the context with correct authorization settings.
        // The sensitive_action function should reject a non-authorized caller.
        let _ = initialize("admin", true);
        let result = sensitive_action("user"); // "user" is not the admin key.
        // For the fixed version, the operation should be rejected.
        assert!(result.is_err(), "Unauthorized access was incorrectly granted");
    }
}