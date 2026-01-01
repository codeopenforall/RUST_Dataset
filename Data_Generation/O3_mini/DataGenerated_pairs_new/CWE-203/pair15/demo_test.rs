#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;
    use std::thread;

    // This test oracle can be run against both implementations.
    // For the vulnerable version, the error messages differ and
    // the assertion below will fail. For the fixed version, the error
    // messages are uniform, and the test will pass.

    // The code below assumes the Manager struct and its verify() method
    // are in scope.

    #[test]
    fn invariant_test() {
        // Creating an instance using the provided Manager API.
        let manager = super::Manager::new();

        // Attempt with a valid record name but wrong secret.
        let result_existing = manager.verify("alice", "WrongSecret");
        // Attempt with a non-existent record.
        let result_nonexistent = manager.verify("bob", "AnySecret");

        // Both should return the same error message.
        let err_existing = result_existing.unwrap_err();
        let err_nonexistent = result_nonexistent.unwrap_err();

        // The invariant: error messages are identical.
        assert_eq!(err_existing, err_nonexistent, "Error messages must be uniform to avoid state disclosure");
    }
}