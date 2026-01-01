//////////////////////////////////////////////
// Test Oracle Function
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::AuthSystem;
    #[test]
    fn test_generic_error_response() {
        let auth = AuthSystem::new();
        auth.register("alice", "secret");
        // Using an incorrect password should result in a uniform error message.
        let result = auth.verify("alice", "wrong");
        match result {
            Err(msg) => {
                // The fixed implementation returns "Authentication failed" for all errors.
                // In the vulnerable code, the error message would be different ("Incorrect password for user").
                assert_eq!(msg, "Authentication failed", "Error message is not generic!");
            },
            Ok(_) => panic!("Authentication unexpectedly succeeded."),
        }
    }
}