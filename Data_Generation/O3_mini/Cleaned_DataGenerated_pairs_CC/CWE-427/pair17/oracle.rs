#[cfg(test)]
mod tests {
    use std::env;
    use std::panic;
    #[test]
    pub fn test_oracle() {
        env::set_var("MALICIOUS_PATH", "malicious_dir");
        let result = panic::catch_unwind(|| {
            super::main();
        });
        match env::var("TEST_MODE").as_deref() {
            Ok("vulnerable") => {
                assert!(result.is_err(), "Expected failure in vulnerable variant");
            },
            Ok("fixed") => {
                assert!(result.is_ok(), "Expected success in fixed variant");
            },
            _ => {
                assert!(result.is_ok(), "Expected success in fixed variant by default");
            }
        }
    }
}
