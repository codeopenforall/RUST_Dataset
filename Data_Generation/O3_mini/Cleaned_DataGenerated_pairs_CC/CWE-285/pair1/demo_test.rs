#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_control() {
        // Create a user with insufficient privileges.
        let account = UserAccount {
            role: UserRole::Regular,
            secret: "Test Secret".to_string(),
        };

        // The API is expected to enforce authorization. For a non-admin,
        // the fixed version should return an error while the vulnerable version
        // would incorrectly return the secret.
        match account.retrieve_info() {
            Ok(data) => {
                // If sensitive data is returned, then the authorization check failed.
                // This assertion fails when "Test Secret" is returned.
                assert_ne!(data, "Test Secret", "Non-admin should not access sensitive data");
            },
            Err(err) => {
                assert_eq!(err, "Access denied", "Expected access denial for non-admin users");
            }
        }
    }
}