        let candidate = self.root.join(file).canonicalize()?;
        let canonical_root = self.root.canonicalize()?;
        if !candidate.starts_with(&canonical_root) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Path traversal detected"));
        }
        let bytes = fs::read(&candidate)?;
        Ok(content) => println!("Unexpected access: Read content: {}...", &content[0..30.min(content.len())]),
        Err(e) => println!("Access blocked: {}", e),
