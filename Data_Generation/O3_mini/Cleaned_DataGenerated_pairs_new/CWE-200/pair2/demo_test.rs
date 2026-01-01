#[cfg(test)]
mod tests {
    use super::run_session;

    #[test]
    fn test_sensitive_data_not_leaked() {
        let output = run_session();
        // The fixed code masks the secret API key.
        // For the vulnerable version, this assertion would fail because "SECRET123" appears in the log.
        assert!(
            !output.contains("SECRET123"),
            "Sensitive information leaked in logs!"
        );
        // Additionally, ensure that masked text is present.
        assert!(
            output.contains("********"),
            "Masked secret was not found in logs!"
        );
    }
}