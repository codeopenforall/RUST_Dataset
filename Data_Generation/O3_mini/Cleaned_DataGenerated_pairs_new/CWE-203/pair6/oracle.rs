#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authentication() {
        let db = Database::new();
        db.add_user("alice", "secret123");
        let res = unsafe { db.check_credentials("alice", "wrongpassword") };
        match res {
            Err(e) => {
                assert!(
                    !e.contains("secret123"),
                    "Sensitive information leaked in error message"
                );
            }
            Ok(_) => {
                panic!("Authentication should fail with incorrect credentials");
            }
        }
    }
}
