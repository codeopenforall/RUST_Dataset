        let canonical_base = fs::canonicalize(&self.base)?;
        let canonical_full = fs::canonicalize(&full_path)?;
        if !canonical_full.starts_with(&canonical_base) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Access outside allowed directory denied",
            ));
        }
