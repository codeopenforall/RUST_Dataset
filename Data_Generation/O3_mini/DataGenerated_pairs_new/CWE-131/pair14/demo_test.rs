#[cfg(test)]
mod tests {
    use super::Handler;
    
    #[test]
    fn test_buffer_copy() {
        // The input string is deliberately chosen to exceed the size of a fat pointer.
        let input = "This is a test string that exceeds the typical pointer size";
        let result = Handler::process(input);
        // Expect the full string bytes to be copied.
        assert_eq!(result, input.as_bytes(), "Buffer copy does not match input");
    }
}