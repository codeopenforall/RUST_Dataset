// Test Oracle Function (separate from the main implementations)
// This test is designed to fail against the vulnerable implementation and pass
// against the corrected version. No modifications are made between runs.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
        // Using a known account with an incorrect password.
        let auth = AccessControl::new();
        let res = auth.authenticate("alice", "wrongpass");
        // The test expects the generic error message.
        assert_eq!(res, Err("Invalid credentials"));
    }
}