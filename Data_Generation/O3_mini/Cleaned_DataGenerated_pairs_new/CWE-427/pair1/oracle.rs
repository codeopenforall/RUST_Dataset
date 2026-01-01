#[cfg(test)]
pub mod tests {
    use std::env;
    pub fn test_process(entry: fn() -> bool) {
        env::set_var("CUSTOM_PATH", "/nonexistent_directory");
        let result = entry();
        assert!(
            result,
            "Test Failure: Process did not spawn as expected (likely using attacker controlled PATH)"
        );
    }
    #[test]
    fn run_test() {
        extern "Rust" {
            fn run_app() -> bool;
        }
        unsafe { test_process(run_app) }
    }
}
