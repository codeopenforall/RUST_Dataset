#[cfg(test)]
mod tests {
    use super::process;
    #[test]
    fn test_buffer_integrity() {
        let result = process();
        assert_eq!(result, 0, "Invariant violation: unexpected buffer modification detected");
    }
}
