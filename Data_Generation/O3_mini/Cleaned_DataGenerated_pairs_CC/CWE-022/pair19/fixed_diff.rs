        let candidate_path = self.base_dir.join(filename);
        let canonical_candidate = fs::canonicalize(&candidate_path)
            .map_err(|e| format!("Error canonicalizing candidate: {}", e))?;
        let canonical_base = fs::canonicalize(&self.base_dir)
            .map_err(|e| format!("Error canonicalizing base: {}", e))?;
        if !canonical_candidate.starts_with(&canonical_base) {
            return Err("Access denied: Path traversal attempt detected".into());
        }
        fs::read_to_string(&canonical_candidate).map_err(|e| format!("Error: {}", e))
