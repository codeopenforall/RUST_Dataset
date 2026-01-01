#[cfg(test)]
mod tests {
    use super::execute;
    #[test]
    fn test_execute() {
        let result = execute();
        assert_eq!(result, 123, "Expected result to be 123, indicating safe resource access");
    }
}
