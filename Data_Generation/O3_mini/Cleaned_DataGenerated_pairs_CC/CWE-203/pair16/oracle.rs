#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authentication_uniform_error() {
        let system = AuthSystem::new();
        let error_existing = system.authenticate("alice", "wrongpass").err().unwrap();
        let error_nonexistent = system.authenticate("bob", "any").err().unwrap();
        assert_eq!(format!("{:?}", error_existing), format!("{:?}", error_nonexistent));
    }
}
