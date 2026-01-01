//////////////// Test Oracle ////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_app_run_error() {
        // Ensure the environment does NOT trigger a valid configuration.
        env::remove_var("TRIGGER");
        let result = app_run();
        // For the fixed code, proper error propagation causes app_run() to return an error.
        // For the vulnerable code, the error is swallowed and a default value (likely 0) is returned.
        assert!(
            result.is_err(),
            "Expected an error due to failed configuration load, but got a valid result: {:?}",
            result.ok()
        );
    }
}