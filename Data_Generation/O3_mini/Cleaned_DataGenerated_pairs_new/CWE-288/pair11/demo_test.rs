//////////////////////////////
// Test Oracle Function     //
// One test, one contract.
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_authentication() {
        // Create an instance with debug mode enabled.
        let auth = Arc::new(Mutex::new(Authenticator::new("securepassword", true)));
        // Acquire the lock and verify using an incorrect password.
        let result = {
            let lock = auth.lock().unwrap();
            lock.verify("badpass")
        };
        // The test asserts that authentication should fail.
        // For the vulnerable version, this test will fail because it returns true (bypassed).
        // For the fixed version, the test passes as the provided password is incorrect.
        assert!(!result, "Authentication should fail with wrong credentials.");
    }
}