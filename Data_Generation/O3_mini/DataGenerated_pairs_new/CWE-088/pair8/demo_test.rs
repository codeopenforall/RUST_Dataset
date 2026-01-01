//////////////////////////////////////////////
// Test oracle to validate command handling behavior.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Executor;

    #[test]
    fn command_injection_test() {
        // Crafted input contains shell metacharacters that could lead to injection
        // In the vulnerable version, these would be interpreted by the shell, causing extra commands to run.
        // In the fixed version, the input is treated as a literal argument to echo.
        let injected_input = "safe_text; echo INJECTED";
        let instance = Executor::new(injected_input.to_string());
        let result = instance.trigger().expect("Failed to execute command");
        let normalized = result.trim();

        // Expected result for the fixed version:
        // The echo command receives the entire string as a literal and prints it without interpreting ';'
        let expected = "safe_text; echo INJECTED";
        assert_eq!(normalized, expected, "Command injection vulnerability detected");
    }
}