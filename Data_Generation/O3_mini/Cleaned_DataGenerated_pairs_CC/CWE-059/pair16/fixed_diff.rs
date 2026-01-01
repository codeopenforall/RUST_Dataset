        if !resolved.starts_with(&self.base) {
            return Err("Access denied: resolved path is outside of the allowed directory".to_string());
        }
