#[cfg(test)]
mod tests {
    use super::Handler;

    #[test]
    fn test_operation() {
        let input = vec![10u8, 20, 30, 40, 50];
        let handler = Handler::new(&input);
        // Expected behavior: the operation skips the first element.
        let result = unsafe { handler.operate() };
        let expected = vec![20, 30, 40, 50];
        assert_eq!(result, expected, "The output does not match the expected result.");
    }
}