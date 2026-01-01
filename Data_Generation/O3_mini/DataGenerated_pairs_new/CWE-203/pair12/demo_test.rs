///////////////////////////////////////////////////////////////
// Test Oracle Function
// This test oracle verifies that authentication failures consistently return
// the unified error message ("Invalid credentials") regardless of whether the
// username exists or the password is incorrect. For the vulnerable version, this
// test will fail due to inconsistent error messages.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    // Import the appropriate items from the module containing the implementation.
    use super::*;

    #[test]
    fn test_authentication_error_message() {
        let db = Database::new();

        // Test with a valid username but incorrect password.
        let err_msg1 = verify(&db, "alice", "wrongpass").err().unwrap();
        // Test with an invalid username.
        let err_msg2 = verify(&db, "bob", "any_pass").err().unwrap();

        // Both calls should produce the same error message.
        assert_eq!(err_msg1, "Invalid credentials", "Unexpected error message for invalid password");
        assert_eq!(err_msg2, "Invalid credentials", "Unexpected error message for non-existent user");
    }
}