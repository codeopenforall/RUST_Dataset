    fn is_safe_path(path: &str) -> bool {
        path.split(':').all(|p| p.starts_with('/'))
    }
        let safe_path = if Self::is_safe_path(&untrusted) {
            untrusted
        } else {
            "/usr/bin".to_string()
        };
        command.env("PATH", safe_path);
