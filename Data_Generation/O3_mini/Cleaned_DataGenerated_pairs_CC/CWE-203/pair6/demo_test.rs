#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authentication() {
        // Initialize the database and add a single user.
        let db = Database::new();
        db.add_user("alice", "secret123");

        // This test checks that when wrong credentials are provided, the error message
        // does not include the sensitive expected password.
        let res = unsafe { db.check_credentials("alice", "wrongpassword") };
        match res {
            Err(e) => {
                // The vulnerable version would include the actual password ("secret123") in the error.
                // The fixed version returns a generic message.
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