    fn get_secure_path(&self, input: &str) -> io::Result<PathBuf> {
        let candidate = self.base.join(input).canonicalize()?;
        let base_canonical = self.base.canonicalize()?;
        if candidate.starts_with(&base_canonical) {
            Ok(candidate)
        } else {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Path traversal attempt detected",
            ))
        }
        let secure_path = self.get_secure_path(input)?;
        let path_clone = secure_path.clone();
