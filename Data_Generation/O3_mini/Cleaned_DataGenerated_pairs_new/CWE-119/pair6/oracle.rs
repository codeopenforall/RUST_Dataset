#[cfg(test)]
mod tests {
    use super::process_operation;
    #[test]
    fn test_memory_bounds() {
        let op_result = process_operation(10);
        assert!(
            op_result.is_err(),
            "Test failed: Operation should not succeed with an out-of-bound index."
        );
    }
}
