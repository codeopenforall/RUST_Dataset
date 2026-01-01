#[cfg(test)]
mod tests {
    use std::panic;
    pub fn run_main() {
        crate::main();
    }
    #[test]
    fn test_double_free() {
        let result = panic::catch_unwind(|| {
            run_main();
        });
        if cfg!(feature = "vulnerable") {
            assert!(result.is_err(), "Expected panic due to double free in vulnerable version");
        } else {
            assert!(result.is_ok(), "Did not expect panic in corrected version");
        }
    }
}
