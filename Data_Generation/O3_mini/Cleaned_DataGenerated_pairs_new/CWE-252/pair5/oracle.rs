#[cfg(test)]
mod tests {
    use std::panic;
    #[test]
    fn test_oracle() {
        let result = panic::catch_unwind(|| {
            crate::run();
        });
        assert!(
            result.is_err(),
            "Expected a panic on update error, but execution proceeded normally."
        );
    }
}
