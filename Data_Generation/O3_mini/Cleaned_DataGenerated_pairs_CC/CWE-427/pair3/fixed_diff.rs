        let default_safe_path = "/usr/bin:/bin".to_string();
        let user_input = env::var("SEARCH_PATH").unwrap_or_else(|_| default_safe_path.clone());
        let sanitized: Vec<&str> = user_input.split(':').filter(|segment| segment.starts_with('/')).collect();
        let safe_value = if sanitized.is_empty() {
            default_safe_path
        } else {
            sanitized.join(":")
        let safe_clone = safe_value.clone();
            env::set_var("PATH", safe_clone);
