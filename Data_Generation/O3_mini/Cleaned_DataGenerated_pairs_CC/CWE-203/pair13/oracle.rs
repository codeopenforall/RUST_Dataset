#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_auth_error_message() {
        let auth = Auth::new();
        let result = auth.login("alice", "wrong_password");
        assert!(result.is_err());
        let err_msg = result.err().unwrap();
        assert_eq!(err_msg, "invalid credentials");
    }
}
