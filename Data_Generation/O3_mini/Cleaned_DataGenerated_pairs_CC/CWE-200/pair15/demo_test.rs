// Test oracle function: This unit test checks that the error log does not contain the sensitive secret
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_leak() {
        let manager = Manager::new("admin", "supersecret");
        manager.simulate_error();
        let log = manager.generate_error_log();
        // The log must NOT include the sensitive secret string "supersecret".
        assert!(
            !log.contains("supersecret"),
            "Sensitive information leaked in log: {}",
            log
        );
    }
}