//////////////////////////////////////////////////////////////////////////
// Test Oracle
//
// The test below verifies that the implementation does not execute injected commands.
// It uses a triggering input "hello; echo INJECTED". In a vulnerable implementation,
// this input would cause the shell to execute two commands, resulting in multi-line output.
// The oracle asserts that the output is confined to a single line, failing if command injection occurs.
//////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::Executor;

    #[test]
    fn test_injection() {
        let executor = Executor::new("PREFIX".to_string());
        // Triggering input that, if unsafely handled, would invoke an extra command.
        let output = executor.execute("hello; echo INJECTED").expect("Execution failed");

        // When passing arguments safely, the output should be a single line:
        // "PREFIX hello; echo INJECTED"
        let lines: Vec<&str> = output.trim().split('\n').collect();
        assert_eq!(lines.len(), 1, "Command injection detected: output spans multiple lines");
    }
}