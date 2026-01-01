#[cfg(test)]
mod tests {
    use super::run_app;
    #[test]
    fn test_total_computation() {
        let total = run_app();
        assert_eq!(total, 680, "The total computed does not match the expected value (680).");
    }
}
