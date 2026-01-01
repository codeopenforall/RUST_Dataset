#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_authentication_error_generic() {
        let svc = Service::new();
        svc.register("alice", "secret");
        let error_msg = svc.authenticate("alice", "wrongpass").unwrap_err();
        assert_eq!(error_msg, "Invalid credentials", "Oracle: Expected generic error message.");
    }
}
