#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    fn run_test<T: Fn(&str, &str) -> Result<(), String>>(auth_fn: T) {
        let err1 = auth_fn("alice", "wrong_password").unwrap_err();
        let err2 = auth_fn("bob", "whatever").unwrap_err();
        assert_eq!(err1, err2, "Error messages differ, potentially leaking user existence information");
    }
    #[test]
    fn test_authentication_oracle() {
        let system = Credentials::create();
        let auth_closure = |u: &str, p: &str| system.authenticate(u, p);
        run_test(auth_closure);
    }
}
