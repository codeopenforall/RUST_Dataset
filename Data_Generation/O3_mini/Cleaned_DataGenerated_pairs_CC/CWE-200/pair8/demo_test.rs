#[cfg(test)]
mod tests {
    use super::run_app;

    #[test]
    fn secret_leak_test() {
        // The expected behavior is that the log output should not contain the original secret.
        let output = run_app();
        assert!(
            !output.contains("SuperSecret123"),
            "Sensitive information leaked in output!"
        );
    }
}