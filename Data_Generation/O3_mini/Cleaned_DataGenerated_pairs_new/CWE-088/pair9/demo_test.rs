#[cfg(test)]
mod tests {
    use super::run_command;

    #[test]
    fn injection_test() {
        // Malicious input designed to trigger command injection if the input is unsanitized.
        // In a vulnerable implementation ("sh -c" with unsanitized command):
        //   "echo Hello; echo injected" would be executed by the shell, producing output with a line break.
        // In the fixed implementation, the argument is passed literally to "echo", producing:
        //   "Hello; echo injected\n"
        let malicious = "Hello; echo injected";
        let output = run_command(malicious).expect("Execution failed");
        // Expect safe literal output. The vulnerable version would interpret the semicolon and produce injected output.
        assert_eq!(
            output,
            "Hello; echo injected\n",
            "Detected command injection vulnerability"
        );
    }
}