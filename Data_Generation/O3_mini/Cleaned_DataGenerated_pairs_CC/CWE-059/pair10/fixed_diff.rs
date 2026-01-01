        let input_path = Path::new(filename);
        let raw_path = input_path.as_os_str().as_bytes().as_ptr();
            let len = input_path.as_os_str().as_bytes().len();
        let canonical = final_path.canonicalize().map_err(|e| e.to_string())?;
        let canonical_allowed = self.allowed_dir.canonicalize().map_err(|e| e.to_string())?;
        if !canonical.starts_with(&canonical_allowed) {
            return Err("Access denied: file outside allowed directory".into());
        }
        let canonical_clone = canonical.clone();
            if let Ok(data) = fs::read_to_string(&canonical_clone) {
