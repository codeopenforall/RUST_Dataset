        let canon = fs::canonicalize(&full_path).map_err(|e| e.to_string())?;
        if !canon.starts_with(&self.base) {
            return Err("Symlink escapes allowed directory".into());
        let mut file = File::open(canon).map_err(|e| e.to_string())?;
