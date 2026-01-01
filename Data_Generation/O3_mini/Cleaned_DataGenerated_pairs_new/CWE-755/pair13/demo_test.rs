#[cfg(test)]
mod tests {
    use super::process_input;

    #[test]
    fn test_error_handling() {
        // The test oracle calls process_input with an invalid input.
        // For a correct implementation, it must return an error.
        let result = process_input("invalid");
        assert!(
            result.is_err(),
            "Expected process to error on invalid input but got Ok({:?})",
            result.ok()
        );
    }
}