        let joined_path = self.base.join(user_input);
        let canonical_base = fs::canonicalize(&self.base)
            .map_err(|e| format!("Error canonicalizing base: {:?}", e))?;
        let canonical_path = fs::canonicalize(&joined_path)
            .map_err(|e| format!("Error canonicalizing path: {:?}", e))?;
        if !canonical_path.starts_with(&canonical_base) {
            return Err("Access to the requested file is not allowed".to_string());
        }
            let path_str = canonical_path.to_str().ok_or_else(|| "Invalid path".to_string())?;
