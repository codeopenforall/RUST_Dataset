#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_update_behavior() {
        let locker = Locker::new();
        // Simulate attacker-controlled input with an out-of-bound index.
        let result = panic::catch_unwind(|| {
            // In the vulnerable version, this call would panic,
            // while in the corrected version it returns an error.
            // To ensure no panic occurs, we call update and ignore its return value.
            let _ = locker.update(15, 42);
        });
        assert!(result.is_ok(), "The update function panicked, indicating a vulnerability.");
    }
}