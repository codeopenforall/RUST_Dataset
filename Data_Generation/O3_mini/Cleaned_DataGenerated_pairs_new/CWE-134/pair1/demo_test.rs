#[cfg(test)]
mod tests {
    use super::Handler;

    // The oracle function tests the processing behavior.
    // For the fixed code, the output must equal the original user payload exactly.
    fn oracle<F: Fn(&str) -> String>(process_fn: F) {
        // Malicious payload containing format specifiers.
        let malicious_input = "Test payload: %x %x";
        let result = process_fn(malicious_input);
        // In the fixed version, using the fixed format specifier ensures that the output is exactly the payload.
        assert_eq!(result, "Test payload: %x %x", "Output did not match expected safe string.");
    }

    #[test]
    fn test_safety() {
        let handler = Handler::new();
        oracle(|input| handler.process(input));
    }
}