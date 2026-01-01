#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authentication_error_message() {
        let manager = AuthManager::new();
        manager.add_user(User {
            username: "alice".to_string(),
            password: "secret123".to_string(),
            secret: "ALICE_SECRET".to_string(),
        });
        let result = manager.verify("alice", "wrongpwd");
        match result {
            Ok(_) => panic!("Authentication should have failed"),
            Err(err) => {
                assert_eq!(err, "Authentication failed: invalid credentials");
            }
        }
    }
}
