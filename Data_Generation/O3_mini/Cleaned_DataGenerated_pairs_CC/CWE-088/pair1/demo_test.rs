//////////////////////////////////////////////
// Test Oracle Function for Command Injection
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::execute_command;

    #[test]
    fn test_command_injection() {
        // This input contains a semicolon and the string "echo Injected".
        // In the insecure implementation, the shell would interpret this as two commands,
        // resulting in output like "Hello\nInjected\n". The safe version will treat the input
        // as a single literal argument, outputting "Hello;echo Injected\n".
        let injection_input = "Hello;echo Injected";
        let output = execute_command(injection_input).expect("Execution failed");

        // The safe execution should preserve the semicolon; it should not split the command.
        // Thus, a vulnerable implementation would not include the semicolon in the output.
        assert!(output.contains(";echo Injected"), "OS Command Injection vulnerability detected!");
    }
}