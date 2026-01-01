#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization() {
        let handler = Handler::new("secret_data");
        let unauthorized_user = User {
            username: "user".into(),
            is_admin: false,
        };

        // In a secure implementation, an unauthorized user must not retrieve the secret.
        // This test will fail if the method incorrectly allows access.
        let result = handler.get_secret(&unauthorized_user);
        assert!(result.is_err(), "Authorization failure: unauthorized user should not access the secret");
    }
}