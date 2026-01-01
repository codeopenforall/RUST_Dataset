#[cfg(test)]
mod tests {
    use std::env;
    use super::Application;
    #[test]
    fn test_path_resolution() {
        env::set_var("MALICIOUS_PATH", "malicious/path");
        let app = Application;
        let resolved = app.resolve_path();
        assert_eq!(resolved, "/usr/bin:/bin", "Resolved PATH should be safe even if MALICIOUS_PATH is set maliciously");
    }
}
