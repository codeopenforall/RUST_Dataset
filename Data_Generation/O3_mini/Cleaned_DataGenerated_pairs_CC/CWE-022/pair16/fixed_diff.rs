        let canonical_base = fs::canonicalize(base).unwrap_or_else(|_| PathBuf::from(base));
            base: canonical_base,
        let joined = self.base.join(input);
        let canonical_path = fs::canonicalize(&joined).map_err(|e| e.to_string())?;
        if !canonical_path.starts_with(&self.base) {
            return Err("Invalid path: Access outside base directory denied".to_string());
        let mut files: Vec<String> = Vec::new();
        let entries = fs::read_dir(&canonical_path).map_err(|e| e.to_string())?;
        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_name = entry.file_name().into_string().unwrap_or_default();
            files.push(file_name);
        }
        Ok(files)
