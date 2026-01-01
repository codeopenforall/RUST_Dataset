        Self { allow_external: false }
        if input.contains("<!ENTITY") {
            return Err("External entity resolution is disabled".to_string());
        Ok(input.to_string())
