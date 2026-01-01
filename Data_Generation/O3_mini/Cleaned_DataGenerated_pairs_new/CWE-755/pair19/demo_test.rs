////////////////////////////////////////////
// Test Oracle (Unit Test)
////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // The oracle test verifies that the application does not panic when configuration errors
    // are properly handled. It removes CONFIG_VAL so that the vulnerable version (if present)
    // would panic due to division by zero, whereas the fixed version returns an error gracefully.
    #[test]
    fn test_oracle() {
        // Ensure the environment variable is not set.
        std::env::remove_var("CONFIG_VAL");
        
        // Execute in a catch_unwind block to detect panics.
        let result = panic::catch_unwind(|| {
            // In the fixed version, execute() returns a Result and does not panic.
            // In the vulnerable version, execute() will panic due to a division by zero.
            execute();
        });
        assert!(
            result.is_ok(),
            "Application panicked, indicating improper handling of exceptional conditions."
        );
    }
}