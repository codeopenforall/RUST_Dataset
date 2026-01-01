#[cfg(test)]
mod tests {
    use super::run_concurrent;
    #[test]
    fn test_processing() {
        let result = run_concurrent();
        assert_eq!(result, 200, "Concurrent processing sum mismatch");
    }
}
