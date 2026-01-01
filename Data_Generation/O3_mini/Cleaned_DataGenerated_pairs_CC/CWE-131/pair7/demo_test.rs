#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // This test targets the buffer copy vulnerability.
    // When using the flawed implementation, copying more bytes than the actual string length
    // results in undefined behavior and an incorrect string output.
    // The oracle asserts that the retrieved content exactly matches the input, which fails for the vulnerable version.
    fn test_buffer_copy() {
        let input = "short";  // 5 bytes only
        let handler = DataHandler::create(input);
        assert_eq!(handler.get_content(), "short");
    }
}