#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn test_execution_path() {
        env::set_var("ATTACKER_PATH", "/malicious");
        let manager = ExecManager::new();
        let result = manager.spawn_process();
        assert!(result.is_ok(), "Process execution should succeed with safe PATH configuration");
    }
}
