#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    fn contract_output<F: Fn() -> Result<String, String>>(executor: F) {
        let output = executor().unwrap_or_else(|err| err);
        let trimmed = output.trim();
        assert_eq!(trimmed, "hello", "The command output did not match the expected value.");
    }
    #[test]
    fn test_insecure() {
        let result = Executor::launch();
        if let Ok(out) = result {
            assert_ne!(out.trim(), "hello", "Vulnerable version should not reliably output 'hello'.");
        } else {
            assert!(true, "Vulnerable version errored as expected.");
        }
    }
    #[test]
    fn test_secure() {
        let result = SafeExecutor::launch();
        let output = result.expect("Secure execution failed unexpectedly.");
        assert_eq!(output.trim(), "hello", "Secure version did not output the expected value.");
    }
}
