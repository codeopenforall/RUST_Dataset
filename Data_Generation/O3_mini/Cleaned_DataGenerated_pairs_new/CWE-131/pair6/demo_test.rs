#[cfg(test)]
mod tests {
    // Import the appropriate module depending on which implementation is being linked.
    // The test must be run against the vulnerable binary (expected to fail)
    // and against the fixed binary (expected to pass).
    use super::*;

    #[test]
    fn test_process_function() {
        // Use an input where the correct length is more than the size of a reference (typically 8 bytes).
        let input = "0123456789"; // 10 bytes in ASCII
        let expected = input.as_bytes();

        let output = process(input);

        // The vulnerable version will only copy 8 bytes whereas the fixed version copies all 10 bytes.
        assert_eq!(output, expected, "The processed output should match the expected full input bytes.");
    }
}