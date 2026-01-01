#[cfg(test)]
mod tests {
    use super::do_process;
    #[test]
    fn test_memory_bounds() {
        let input = vec![1u8; 20];
        let result = do_process(&input);
        assert!(result.is_err(), "Expected error on oversized input, but got a valid result");
    }
}
