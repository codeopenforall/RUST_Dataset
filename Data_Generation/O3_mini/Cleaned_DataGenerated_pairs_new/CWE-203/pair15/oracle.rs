#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;
    use std::thread;
    #[test]
    fn invariant_test() {
        let manager = super::Manager::new();
        let result_existing = manager.verify("alice", "WrongSecret");
        let result_nonexistent = manager.verify("bob", "AnySecret");
        let err_existing = result_existing.unwrap_err();
        let err_nonexistent = result_nonexistent.unwrap_err();
        assert_eq!(err_existing, err_nonexistent, "Error messages must be uniform to avoid state disclosure");
    }
}
