#[cfg(test)]
mod tests {
    use super::compute;

    #[test]
    fn division_oracle() {
        // Test input that triggers division by zero.
        let result = compute(100, 0);
        // Fixed version is expected to return a controlled error.
        // Vulnerable version returns a different error message.
        assert_eq!(result, Err("Division by zero error"), "Expected controlled error on division by zero.");
    }
}