#[cfg(test)]
mod tests {
    use super::Manager;

    // Test oracle: For a non-admin role, the call should return an error.
    #[test]
    fn test_oracle() {
        // Expect an error for unauthorized user in the improved code.
        let result = Manager::allocate_resources("user", 5);
        assert!(result.is_err(), "Unauthorized role should result in an error.");
    }
}