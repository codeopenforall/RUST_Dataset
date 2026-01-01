//////////////////// Test Oracle Begin ////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_uniform_error() {
        // For both an existing user with the wrong password and a non-existent user,
        // the error message must be identical to avoid sensitive state disclosure.
        let system = AuthSystem::new();
        let error_existing = system.authenticate("alice", "wrongpass").err().unwrap();
        let error_nonexistent = system.authenticate("bob", "any").err().unwrap();

        // The test expects a unified error (e.g., "Failure") for all authentication failures.
        assert_eq!(format!("{:?}", error_existing), format!("{:?}", error_nonexistent));
    }
}
//////////////////// Test Oracle End ////////////////////