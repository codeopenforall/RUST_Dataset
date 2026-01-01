#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::panic;
    // For testing, expect the Secure struct from the current module.
    // When compiled with vulnerable code, the panic message will include the secret.
    // With the fixed code, the secret will not be present.
    // NOTE: Replace `crate::Secure` with the appropriate module path if necessary.
    use crate::Secure;

    // Helper function to capture panic messages.
    fn capture_panic<F: FnOnce() + std::panic::UnwindSafe>(f: F) -> String {
        match panic::catch_unwind(f) {
            Ok(_) => "No panic".to_string(),
            Err(err) => {
                if let Some(s) = err.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = err.downcast_ref::<String>() {
                    s.to_string()
                } else {
                    "Panic with non-string payload".to_string()
                }
            }
        }
    }

    #[test]
    fn test_sensitive_not_leaked() {
        let secure = Arc::new(Secure::new("SuperSecretPassword".to_string()));
        // Use an index that forces the error condition.
        let res = capture_panic(|| {
            secure.run_task(100);
        });
        // In the vulnerable version, res will include "SuperSecretPassword".
        // In the fixed version, res should not include it.
        assert!(
            !res.contains("SuperSecretPassword"),
            "Test failed: Sensitive data was exposed in error message."
        );
    }
}