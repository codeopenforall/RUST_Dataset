#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_shell_injection() {
        // Input containing shell metacharacters intended to inject an extra command.
        let injection_input = "Hello; echo INJECTED";
        let result = run(injection_input).expect("Command execution failed");
        // The fixed version, which avoids shell interpretation, will print the argument as-is.
        // Expected safe behavior: the output should be exactly "Hello; echo INJECTED\n"
        // The vulnerable version would execute "echo Hello; echo INJECTED" and produce "Hello\nINJECTED\n"
        assert_eq!(result, "Hello; echo INJECTED\n");
    }
}