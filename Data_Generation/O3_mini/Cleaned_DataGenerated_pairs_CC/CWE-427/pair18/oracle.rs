#[cfg(test)]
mod tests {
    use std::env;
    use super::do_action;
    #[test]
    fn test_search_path() {
        env::set_var("PATH", "/tmp/malicious_dir");
        let result = do_action();
        assert!(result, "Command lookup should succeed using the secure search path.");
    }
}
