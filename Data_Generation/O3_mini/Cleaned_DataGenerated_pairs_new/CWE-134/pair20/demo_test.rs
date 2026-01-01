#[cfg(test)]
mod tests {
    use super::Formatter;

    // The test uses a sample input containing a format specifier.
    // For the vulnerable code, the "{}" is replaced with the secret,
    // producing "Hello, SensitiveData". In the fixed version, the input is sanitized,
    // so the output should be "Hello, {{}}SensitiveData".
    //
    // This test acts as our oracle:
    // - It fails for the vulnerable implementation because the output does not match
    //   the expected literal interpretation.
    // - It passes for the fixed implementation.
    #[test]
    fn test_format_sanitization() {
        let formatter = Formatter {
            secret: "SensitiveData".to_owned(),
        };
        let input = "Hello, {}";
        let output = formatter.run(input);
        let expected = "Hello, {{}}SensitiveData".to_owned();
        assert_eq!(output, expected, "The formatted output does not match the expected sanitized result.");
    }
}