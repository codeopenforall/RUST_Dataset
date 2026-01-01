#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_api() {
        let result = execute();
        // The expected output should match exactly the allocated vector data.
        assert_eq!(result, vec![1u8, 2, 3, 4], "The output buffer does not match the expected content");
    }
}