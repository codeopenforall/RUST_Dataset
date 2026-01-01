#[cfg(test)]
mod tests {
    use std::env;
    use super::Executor;
    #[test]
    fn test_path_security() {
        env::set_var("UNTRUSTED_PATH", "malicious");
        let executor = Executor::new();
        let output = executor.run("env").expect("Failed to run env command");
        assert!(output.contains("PATH=/usr/bin"),
                "Unsafe PATH detected! Output: {}",
                output);
    }
}
