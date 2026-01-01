#[cfg(test)]
mod tests {
    use super::process;
    #[test]
    fn test_process_value() {
        let result = process();
        assert_eq!(result, 42, "Process function did not return the expected value 42.");
    }
}
