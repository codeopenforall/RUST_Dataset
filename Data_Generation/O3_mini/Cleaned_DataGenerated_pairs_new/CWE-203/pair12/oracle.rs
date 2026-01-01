#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authentication_error_message() {
        let db = Database::new();
        let err_msg1 = verify(&db, "alice", "wrongpass").err().unwrap();
        let err_msg2 = verify(&db, "bob", "any_pass").err().unwrap();
        assert_eq!(err_msg1, "Invalid credentials", "Unexpected error message for invalid password");
        assert_eq!(err_msg2, "Invalid credentials", "Unexpected error message for non-existent user");
    }
}
