#[cfg(test)]
mod tests {
    use std::io;
    // The function run_concurrently is assumed to be available from the respective module.
    // This test oracle verifies that the command execution returns exactly the provided input as output.
    // For the test, we use a string with shell metacharacters that vulnerable code would inadvertently execute.
    //
    // Expected safe behavior: the output should be "safe_test; echo INJECTED\n" because the argument
    // is passed directly to "echo" without shell interpolation.
    //
    // In contrast, the vulnerable version will execute "echo safe_test; echo INJECTED" resulting in:
    // "safe_test\nINJECTED\n", causing the test to fail.
    const EXPECTED: &str = "safe_test; echo INJECTED\n";

    #[test]
    fn test_command_safety() -> io::Result<()> {
        // Injection input that can trigger multiple commands in a shell.
        let injection = "safe_test; echo INJECTED".to_string();
        let result = crate::run_concurrently(injection)?;

        // Assert that the output exactly matches the expected safe string.
        // Note: The vulnerable implementation would produce a different output.
        assert_eq!(result, EXPECTED, "Command execution output did not match expected safe output");
        Ok(())
    }
}