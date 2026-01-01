        let canonical = user_path.canonicalize()?;
        if !canonical.starts_with(&self.allowed) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied: Invalid path"));
        fs::write(canonical, data)
