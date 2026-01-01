use std::path::Path;
            command: "/usr/bin/example_app".to_string(),
        if !self.is_secure() {
            return Err("Insecure command path detected".to_string());
        }
        Path::new(&self.command).is_absolute()
