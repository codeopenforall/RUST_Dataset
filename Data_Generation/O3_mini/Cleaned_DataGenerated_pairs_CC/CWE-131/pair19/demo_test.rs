#[cfg(test)]
mod tests {
    use super::Formatter;

    #[test]
    fn test_process_output() {
        let formatter = Formatter;
        // Provide an input that exceeds the size of a String on most systems.
        let input = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAA"; // 29 bytes (likely > size_of::<String>())
        unsafe {
            let result = formatter.process(input);
            // Expected: input bytes appended with one null terminator.
            let mut expected = input.as_bytes().to_vec();
            expected.push(0);
            assert_eq!(result, expected, "The serialized output does not match the expected C-string format");
        }
    }
}