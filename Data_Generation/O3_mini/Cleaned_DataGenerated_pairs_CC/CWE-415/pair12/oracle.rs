#[cfg(test)]
mod tests {
    use std::panic;
    #[test]
    fn test_oracle() {
        let result = panic::catch_unwind(|| {
            crate::main();
        });
        #[cfg(feature = "vul")]
        assert!(result.is_err(), "Expected panic due to double free vulnerability");
        #[cfg(not(feature = "vul"))]
        assert!(result.is_ok(), "Execution should succeed without panic");
    }
}
