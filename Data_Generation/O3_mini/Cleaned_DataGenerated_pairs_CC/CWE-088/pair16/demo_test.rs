#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_safety() {
        let runner = CmdRunner::new();
        // The input includes shell metacharacters that, if interpreted,
        // would cause an injected command to execute.
        // The expected safe output is a literal echo of the string with a newline.
        let input = "test; echo injected";
        let output = runner.run(input).expect("Execution should succeed");
        let expected = "test; echo injected\n";
        assert_eq!(
            output, expected,
            "Output did not match expected safe literal output"
        );
    }
}