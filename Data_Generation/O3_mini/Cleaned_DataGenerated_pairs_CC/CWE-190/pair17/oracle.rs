#[cfg(test)]
mod tests {
    use super::run;
    #[test]
    fn test_overflow() {
        let input = 1_073_741_825;
        let result = run(input);
        assert!(result.is_err(), "Expected error due to allocation size overflow");
    }
}
