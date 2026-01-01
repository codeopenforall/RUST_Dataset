        if !custom.is_empty() && custom.starts_with("/usr/bin") {
            if let Ok(valid) = std::str::from_utf8(custom.as_bytes()) {
                return valid.to_string();
            }
        String::from("/usr/bin:/bin")
        cmd.env("PATH", chosen_path);
