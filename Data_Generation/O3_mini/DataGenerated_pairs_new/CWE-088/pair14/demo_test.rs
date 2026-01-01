#[cfg(test)]
mod tests {
    use super::simulate;

    #[test]
    fn check_command_output() {
        // For the safe implementation the output should be a literal echo of the input,
        // including metacharacters. In the insecure implementation, the shell would interpret
        // the metacharacters and execute an extra command ("echo injection").
        let output = simulate();
        // Trim to remove any trailing newline.
        let result = output.trim();
        // The fixed (safe) version should output exactly the given input.
        // Vulnerable version would produce additional output from the injected command.
        assert_eq!(
            result,
            "injected_text; echo injection",
            "Detected command injection vulnerability!"
        );
    }
}