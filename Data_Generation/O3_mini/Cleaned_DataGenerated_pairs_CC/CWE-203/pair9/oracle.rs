#[cfg(test)]
mod tests {
    use super::AuthSystem;
    #[test]
    fn test_generic_error_response() {
        let auth = AuthSystem::new();
        auth.register("alice", "secret");
        let result = auth.verify("alice", "wrong");
        match result {
            Err(msg) => {
                assert_eq!(msg, "Authentication failed", "Error message is not generic!");
            },
            Ok(_) => panic!("Authentication unexpectedly succeeded."),
        }
    }
}
